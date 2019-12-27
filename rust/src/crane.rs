use serial::SystemPort;
use crate::logger::*;
use crate::json_models::*;
use crate::robust_arduino::*;
use std::io;

pub fn handle_crane(mut port: &mut SystemPort, payload_string: &str, debug: bool) {
	let msg: CraneEvent = match serde_json::from_str(&payload_string) {
		Ok(m) => m,
		Err(e) => panic!("Something went wrong with parsing json: {}", e),
	};
	let direction = &msg.config.direction;
	set_crane(&mut port, *direction);
	log(format!("Order received: {:?}", read_order(port).unwrap()));
	if debug {
		debug_crane(&mut port);
	}
}

fn set_crane<T: io::Write>(port: &mut T, direction: i8) {
	write_order(port, Order::CRANE_DIRECTION).unwrap();
	write_i8(port, direction).unwrap();
}

fn debug_crane<T: io::Read>(port: &mut T) {
	log(format!("Order received: {:?}", read_order(port).unwrap()));
	log(format!("Parameter received: {:?}", read_i8(port).unwrap()));
}