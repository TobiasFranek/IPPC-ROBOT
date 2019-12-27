use serial::SystemPort;
use crate::json_models::*;
use crate::robust_arduino::*;
use crate::logger::*;
use std::io;

pub fn handle_motor(mut port: &mut SystemPort, payload_string: &str, debug: bool) {
	let msg: MotorEvent = match serde_json::from_str(&payload_string) {
		Ok(m) => m,
		Err(e) => panic!("Something went wrong with parsing json: {}", e),
	};
	let left = &msg.config.left;
	let right = &msg.config.right;

	set_motor(&mut port, *left, *right);
	log(format!("Order received: {:?}", read_order(port).unwrap()));
	if debug {
		debug_motor(&mut port);
	}
}

fn set_motor<T: io::Write>(port: &mut T, left: i16, right: i16) {
	write_order(port, Order::MOTOR).unwrap();
	write_i16(port, left).unwrap();
	write_i16(port, right).unwrap();
}

fn debug_motor<T: io::Read>(port: &mut T) {
	log(format!("Order received: {:?}", read_order(port).unwrap()));
	log(format!("Parameter received: {:?}", read_i16(port).unwrap()));
	log(format!("Parameter received: {:?}", read_i16(port).unwrap()));
}