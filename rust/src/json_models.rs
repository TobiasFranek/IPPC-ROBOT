use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CraneConfig {
	pub direction: i8,
}

#[derive(Serialize, Deserialize)]
pub struct CraneEvent {
	pub event: String,
	pub config: CraneConfig,
}

#[derive(Serialize, Deserialize)]
pub struct GrabberConfig {
	pub direction: i8,
}

#[derive(Serialize, Deserialize)]
pub struct GrabberEvent {
	pub event: String,
	pub config: GrabberConfig,
}

#[derive(Serialize, Deserialize)]
pub struct MotorConfig {
	pub left: i16,
	pub right: i16,
}

#[derive(Serialize, Deserialize)]
pub struct MotorEvent {
	pub event: String,
	pub config: MotorConfig,
}