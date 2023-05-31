#![warn(rust_2018_idioms, unreachable_pub)]
#![deny(unsafe_code)]
#![forbid(unused_must_use)]
#![no_std]

use bincode::{Decode, Encode};

mod cards;
pub use cards::*;

//todo:
// new structure
// event + game + keep alive message

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
pub enum ToPcProtocol {
	ConnectionResponse
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
pub enum ToPypadeGameEvent {
	NewLevel(AvailableCards)
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum MessageToPyBadge {
	Protocol(ToPybadgeProtocol),
	GameEvent(ToPypadeGameEvent)
}
