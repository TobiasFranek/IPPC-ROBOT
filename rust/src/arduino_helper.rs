use crate::robust_arduino::*;
use crate::logger::*;
use serial::prelude::*;
use serial::SystemPort;
use std::time::Duration;
use std::thread;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud115200,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone,
};

pub fn connect_to_arduino(serial_port: &str, debug: bool) -> SystemPort {
	log(format!("Opening port: {:?}", serial_port));
	let mut port = serial::open(&serial_port).unwrap();
	port.configure(&SETTINGS).unwrap();
	// timeout of 30s
	port.set_timeout(Duration::from_secs(30)).unwrap();

	loop
	{
		log(format!("Waiting for Arduino..."));
		let order = Order::HELLO;
		write_order(&mut port, order).unwrap();
		let received_order = Order::from_i8(read_i8(&mut port).unwrap()).unwrap();
		if received_order == Order::ALREADY_CONNECTED
		{
			break;
		}
		thread::sleep(Duration::from_secs(1));
	}

	log(format!("Connected to Arduino"));
	log(format!("Order received: {:?}", read_order(&mut port).unwrap()));

	log(format!("Attempting to sync Arduino Config..."));
	
	write_order(&mut port, Order::SYNC_CONFIG).unwrap();
	if debug {
		write_i8(&mut port, 1).unwrap();
	} else {
		write_i8(&mut port, 0).unwrap();
	}

	log(format!("Order received: {:?}", read_order(&mut port).unwrap()));
	if debug {
		log(format!("Order received: {:?}", read_order(&mut port).unwrap()));
		log(format!("Parameter received: {:?}", read_i8(&mut port).unwrap()));
	}


	return port;
}