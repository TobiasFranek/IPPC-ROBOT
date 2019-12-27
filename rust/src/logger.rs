extern crate chrono;
use chrono::{DateTime, Utc};

pub fn log(msg: String) {
	let now: DateTime<Utc> = Utc::now();
	print!("[{}]: ", now.to_rfc3339());
	println!("{}", msg);
}