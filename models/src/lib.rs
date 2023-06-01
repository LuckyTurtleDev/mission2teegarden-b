#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use, unsafe_code)]
#![no_std]

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
	NewLevel(AvailableCards),
	GameOver(GameOver)
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum MessageToPyBadge {
	Protocol(ToPybadgeProtocol),
	GameEvent(ToPypadeGameEvent)
}
