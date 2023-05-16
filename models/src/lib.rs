#![no_std]
use bincode::{Decode, Encode};

mod cards;
pub use cards::*;

//todo:
// new structure
// event + game + keep alive message

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum Key {
	Start
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum ToPcProtocol {
	ConnectionResponse
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum ToPcGameEvent {
	KeyPressed(Key)
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
