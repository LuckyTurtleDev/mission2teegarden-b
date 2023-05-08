#![no_std]
use bincode::{Decode, Encode};

mod cards;
pub use cards::*;

#[derive(Debug, Clone, Decode, Encode)]
pub enum MessageToPc {
	ConnectionResponse
}

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum MessageToPyBadge {
	ConnectionRequest,
	ConnectionConfirmation,
	NewLevel(AvailableCards)
}

