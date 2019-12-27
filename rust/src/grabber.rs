use serial::SystemPort;
use crate::logger::*;
use crate::json_models::*;
use crate::robust_arduino::*;
use std::io;

pub fn handle_grabber(mut port: &mut SystemPort, payload_string: &str, debug: bool) {
	let msg: GrabberEvent = match serde_json::from_str(&payload_string) {
		Ok(m) => m,
		Err(e) => panic!("Something went wrong with parsing json: {}", e),
	};
	let direction = &msg.config.direction;
	set_grabber(&mut port, *direction);
	log(format!("Order received: {:?}", read_order(port).unwrap()));
	if debug {
		debug_grabber(&mut port);
	}
}

fn set_grabber<T: io::Write>(port: &mut T, direction: i8) {
	write_order(port, Order::GRABBER).unwrap();
	write_i8(port, direction).unwrap();
}

fn debug_grabber<T: io::Read>(port: &mut T) {
	log(format!("Order received: {:?}", read_order(port).unwrap()));
	log(format!("Parameter received: {:?}", read_i8(port).unwrap()));
}