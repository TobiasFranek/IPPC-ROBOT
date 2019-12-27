//! # Robust Arduino Serial
//!
//! `robust_arduino_serial` is a simple and robust serial communication protocol.
//! It was designed to make two arduinos communicate, but can also be useful
//! when you want a computer (e.g. a Raspberry Pi) to communicate with an Arduino.
//!
//! It works with anything that implement the io::Write and io::Read traits.
//!
//!

use std::io;

#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Order
{
	HELLO = 0,
	CRANE_DIRECTION = 1,
	MOTOR = 2,
	GRABBER = 3,
	ALREADY_CONNECTED = 4,
	ERROR = 5,
	RECEIVED = 6,
	STOP = 7,
	SYNC_CONFIG = 8,
	CLOSE = 9,
	ULTRASONIC = 10
}

/// Convert an int to an Order
///
/// # Example
///
/// ```
/// use robust_arduino_serial::Order;
///
/// let order: i8 = 2;  // Order::MOTOR has the index 2 in the enum
/// let converted_order = robust_arduino_serial::convert_i8_to_order(order).unwrap();
///
/// assert_eq!(converted_order, Order::MOTOR);
/// ```
pub fn convert_i8_to_order(order: i8) -> Option<Order>
{
    match order
    {
        0 => Some(Order::HELLO),
        1 => Some(Order::CRANE_DIRECTION),
		2 => Some(Order::MOTOR),
        3 => Some(Order::GRABBER),
        4 => Some(Order::ALREADY_CONNECTED),
        5 => Some(Order::ERROR),
        6 => Some(Order::RECEIVED),
		7 => Some(Order::STOP),
		8 => Some(Order::SYNC_CONFIG),
		9 => Some(Order::CLOSE),
		10 => Some(Order::ULTRASONIC),
        _ => None
    }
}

/// Shortcut for convert_i8_to_order
///
/// # Example
///
/// ```
/// use robust_arduino_serial::Order;
///
/// let order: i8 = 2;  // Order::MOTOR has the index 2 in the enum
/// let converted_order = Order::from_i8(order).unwrap();
///
/// assert_eq!(converted_order, Order::MOTOR);
/// ```
impl Order {
    pub fn from_i8(num: i8) -> Option<Order> {
        convert_i8_to_order(num)
    }
}

/// Read one byte from a file/serial port and convert it to an order
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use robust_arduino_serial::*;
///
/// let mut buffer = Cursor::new(vec![2]);
/// let order = read_order(&mut buffer).unwrap().unwrap();
/// // Order::MOTOR has the index 2 in the enum
/// assert_eq!(Order::MOTOR, order);
/// ```
pub fn read_order<T: io::Read>(file: &mut T) -> Result<Option<Order>, io::Error>
{
    let order = read_i8(file)?;
    Ok(Order::from_i8(order))
}

/// Read one byte from a file/serial port and convert it to a 8 bits int
///
/// # Example
///
/// ```
/// use std::io::Cursor;
///
/// let mut buffer = Cursor::new(vec![2]);
/// let num: i8 = robust_arduino_serial::read_i8(&mut buffer).unwrap();
///
/// assert_eq!(2, num);
/// ```
pub fn read_i8<T: io::Read>(file: &mut T) -> Result<i8, io::Error>
{
    let mut read_buffer = [0u8; 1];
    file.read_exact(&mut read_buffer)?;
    Ok(read_buffer[0] as i8)
}

/// Read two bytes from a file/serial port and convert it to a 16 bits int
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use std::io::SeekFrom;
/// use std::io::prelude::*;
/// use robust_arduino_serial::*;
///
/// let mut buffer = Cursor::new(Vec::new());
/// let number: i16 = -355;
///
/// // Write the number to the buffer
/// write_i16(&mut buffer, number).unwrap();
///
/// // Go to the beginning of the buffer
/// buffer.seek(SeekFrom::Start(0)).unwrap();
///
/// // Read 16 bits (two bytes) from the buffer
/// let read_number: i16 = robust_arduino_serial::read_i16(&mut buffer).unwrap();
///
/// assert_eq!(read_number, number);
/// ```
pub fn read_i16<T: io::Read>(file: &mut T) -> Result<i16, io::Error>
{
    let mut read_buffer = [0u8; 2];
    file.read_exact(&mut read_buffer)?;
    let number: u16 = ((read_buffer[0] as u16) & 0xff) | ((read_buffer[1] as u16) << 8 & 0xff00);
    Ok(number as i16)
}

/// Read four bytes from a file/serial port and convert it to a 32 bits int
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use std::io::SeekFrom;
/// use std::io::prelude::*;
/// use robust_arduino_serial::*;
///
/// let mut buffer = Cursor::new(Vec::new());
/// let big_number: i32 = 16384; // 2^14
///
/// // Write the number to the buffer
/// write_i32(&mut buffer, big_number);
///
/// // Go to the beginning of the buffer
/// buffer.seek(SeekFrom::Start(0)).unwrap();
///
/// // Read 32 bits (four bytes) from the buffer
/// let read_number: i32 = robust_arduino_serial::read_i32(&mut buffer).unwrap();
///
/// assert_eq!(big_number, read_number);
/// ```
pub fn read_i32<T: io::Read>(file: &mut T) -> Result<i32, io::Error>
{
    let mut read_buffer = [0u8; 4];
    file.read_exact(&mut read_buffer)?;
    let number: u32 = ((read_buffer[0] as u32) & 0xff) | ((read_buffer[1] as u32) << 8 & 0xff00) | ((read_buffer[2] as u32) << 16 & 0xff0000) | ((read_buffer[3] as u32) << 24 & 0xff000000);
    Ok(number as i32)
}

/// Write an order to a file/serial port.
/// It returns the number of bytes written
///
/// # Example
///
/// ```
/// use robust_arduino_serial::write_order;
/// use robust_arduino_serial::Order;
/// let mut buffer = Vec::new();
///
/// // write the order (one byte) to the buffer
/// write_order(&mut buffer, Order::HELLO).unwrap();
/// ```
pub fn write_order<T: io::Write>(file: &mut T, order: Order) -> io::Result<usize>
{
    write_i8(file, order as i8)
}

/// Write one byte int to a file/serial port.
/// It returns the number of bytes written
///
/// # Example
///
/// ```
/// use robust_arduino_serial::write_i8;
/// let mut buffer = Vec::new();
/// let num: i8 = 2;
///
/// // write 8 bits (one byte) to the buffer
/// write_i8(&mut buffer, num).unwrap();
/// ```
pub fn write_i8<T: io::Write>(file: &mut T, num: i8) -> io::Result<usize>
{
    let buffer = [num as u8];
    let num_bytes = file.write(&buffer)?;
    Ok(num_bytes)
}

/// Write two bytes int to a file/serial port.
/// It returns the number of bytes written
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use robust_arduino_serial::*;
///
/// let mut buffer = Cursor::new(Vec::new());
/// let number: i16 = 366;
///
/// // write 16 bits (two bytes) to the buffer
/// write_i16(&mut buffer, number).unwrap();
/// ```
pub fn write_i16<T: io::Write>(file: &mut T, num: i16) -> io::Result<usize>
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8
    ];
    let num_bytes = file.write(&buffer)?;
    Ok(num_bytes)
}

/// Write four bytes int to a file/serial port.
/// It returns the number of bytes written
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use robust_arduino_serial::*;
///
/// let mut buffer = Cursor::new(Vec::new());
/// let big_number: i32 = -16384; // -2^14
///
/// // write 32 bits (four bytes) to the buffer
/// write_i32(&mut buffer, big_number).unwrap();
/// ```
pub fn write_i32<T: io::Write>(file: &mut T, num: i32) -> io::Result<usize>
{
    let buffer = [
        (num & 0xff) as u8,
        (num >> 8 & 0xff) as u8,
        (num >> 16 & 0xff) as u8,
        (num >> 24 & 0xff) as u8
    ];
    let num_bytes = file.write(&buffer)?;
    Ok(num_bytes)
}