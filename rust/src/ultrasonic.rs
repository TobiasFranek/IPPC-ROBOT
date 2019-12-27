use serial::SystemPort;
use crate::robust_arduino::*;
use crate::logger::*;

pub fn get_ultrasonic(mut port: &mut SystemPort, debug: bool) -> i32 {
	log(format!("Fetching Ultrasonic sensor values."));
	write_order(&mut port, Order::ULTRASONIC).unwrap();
	log(format!("Order received: {:?}", read_order(&mut port).unwrap()));
	if debug {
		log(format!("Order received: {:?}", read_order(&mut port).unwrap()));
	}

	let value = read_i32(&mut port).unwrap();

	if debug {
		log(format!("Parameter received: {:?}", value));
	}

	log(format!("Received Ultrasonic sensor value: {:?}", value));

	return value;
}
