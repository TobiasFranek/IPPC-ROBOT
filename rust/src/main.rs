#[allow(dead_code)]
mod robust_arduino;
mod json_models;
mod arduino_helper;
mod crane;
mod grabber;
mod motor;
mod logger;
mod ultrasonic;

extern crate websocket;
extern crate serial;
extern crate serde_json;

use serde_json::{Value};
use std::thread;
use arduino_helper::*;
use robust_arduino::*;
use crane::*;
use grabber::*;
use motor::*;
use ultrasonic::*;
use logger::*;
use websocket::sync::Server;
use websocket::{OwnedMessage};
use websocket::result::WebSocketError;
use websocket::ws::dataframe::DataFrame;
use std::env;
use serde_json::json;

fn main() {
	let debug = env::var("DEBUG").unwrap();
	let debug: bool = debug.parse().unwrap();

	let server = Server::bind("192.168.0.13:2794").unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
		thread::spawn(move || {
			log(format!("New Connection Request."));
			if !request.protocols().contains(&"rust-websocket".to_string()) {
				request.reject().unwrap();
				return;
			}

			let port_string = env::var("PORT").unwrap(); 

			let mut port = connect_to_arduino(&port_string, debug);

			let client = request.use_protocol("rust-websocket").accept().unwrap();

			let ip = client.peer_addr().unwrap();

			log(format!("Connection from {}", ip));
			
			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message: OwnedMessage = match message {
					Ok(m) => m,
					Err(WebSocketError::NoDataAvailable) => return, // connection close by server
					Err(e) => {
						log(format!("Receive Loop First: {}", e));
						break;
					}
				};
		
				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						log(format!("Client {} disconnected", ip));
						write_order(&mut port, Order::CLOSE).unwrap();
						log(format!("Order received: {:?}", read_order(&mut port).unwrap()));
						if debug {
							log(format!("Order received: {:?}", read_order(&mut port).unwrap()));
						}
						return;
					},
					OwnedMessage::Ping(ping) => {
						log(format!("Received Ping!"));
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					},
					_ => {
						let payload_bytes = message.take_payload();
						let payload_string = match String::from_utf8(payload_bytes) {
							Ok(p) => p,
							Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
						};
						
						let message_type: Value = match serde_json::from_str(&payload_string) {
							Ok(m) => m,
							Err(e) => panic!("Something went wrong with parsing json: {}", e),
						};

						let message_type = message_type["event"].as_str().unwrap();

						log(format!("Got {:?} event type.", message_type));

						if message_type == "CRANE_DIRECTION" {
							handle_crane(&mut port, &payload_string, debug);
						} else if message_type == "GRABBER" {
							handle_grabber(&mut port, &payload_string, debug);
						} else if message_type == "MOTOR" {
							handle_motor(&mut port, &payload_string, debug);
						} else if message_type == "ULTRASONIC" {
							let value = get_ultrasonic(&mut port, debug);
							let event = json!({
								"event": "ULTRASONIC",
								"config": {
									"value": value
								}
							});
							let message = OwnedMessage::Text(event.to_string());
							sender.send_message(&message).unwrap();
						}
					}
				}
			}
		});
	}
}