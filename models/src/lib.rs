#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use, unsafe_code)]
#![no_std]

//! This create store models and protcoll data,
//! which is used by the communication between pybadeg and pc.
//!
//! # Protocol:
//! Every message is neither a Protocoll message or a Gamevent.
//! The [`KeepAlive`](`MessageToPc::KeepAlive) is also definded at toplevel, to make it only one byte big,
//! because it is send very frequently to pc.
//! The pc assume that the [`KeepAlive`](`MessageToPc::KeepAlive) message is send at least once every second.
//! Otherwise the pc assume that the pypbadge was disconnected.
//!
//! ### Connection Establishment
//! The pc search for seriell devices at start and does send an [`ConnectionRequest`](ToPybadgeProtocol::ConnectionRequest)
//! to each Seriell devices.
//! The pybadge responds with [`ConnectionResponse`](ToPcProtocol::ConnectionResponse).
//! After sending an [`ConnectionResponse`](ToPcProtocol::ConnectionResponse)
//! the pybadge start listen to other incomming message too.

use bincode::{Decode, Encode};

mod cards;
pub use cards::*;

//todo:
// new structure
// event + game + keep alive message

#[derive(Debug, Clone, Copy, Decode, Encode, PartialEq, Eq)]
pub enum GameOver {
	/// Player has drive outside the map.
	DriveAway,
	/// Player has not reach goal in time.
	TimeOut,
	/// Player has crash.
	Crash
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum Key {
	A,
	B,
	Up,
	Down,
	Left,
	Right,
	Start,
	Select
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub struct Log {
	// data sending/de-/encoding is broken if array is to long.
	// for example if messsage if is 128 long. This output will be send for "hiiiii":
	// message: [104, 105, 105, 105, 105, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2], length: 2 }
	// at some point the bytes become always 2.
	pub message: [u8; 100],
	pub length: u16
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum ToPcProtocol {
	ConnectionResponse,
	Log(Log)
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum ToPcGameEvent {
	KeyPressed(Key),
	/// The solution which the player has created for this level
	// currently heapless::vec is not supported by bincode,
	// so use Array<Option> as workaround.
	// see https://github.com/bincode-org/bincode/issues/643
	Solution([Option<Card>; 12])
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum MessageToPc {
	Protocol(ToPcProtocol),
	GameEvent(ToPcGameEvent),
	///pybadge is still connected and work
	KeepAlive
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum ToPybadgeProtocol {
	ConnectionRequest
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub struct NeoPixelColor {
	pub r: u8,
	pub g: u8,
	pub b: u8
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum ToPypadeGameEvent {
	/// set all NeoPixel to this Color.
	/// Regular used as indication for Player number
	NeoPixelColor(NeoPixelColor),
	NewLevel(AvailableCards),
	GameOver(GameOver),
	/// Retry the current level,
	/// with out clearing the solution of the player
	Retry,
	/// Index of the card which is currently evaluated
	CurrentCardIndex(u8)
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum MessageToPyBadge {
	Protocol(ToPybadgeProtocol),
	GameEvent(ToPypadeGameEvent)
}
