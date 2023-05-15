#![no_std]
use bincode::{Decode, Encode};

mod cards;
pub use cards::*;

#[derive(Debug, Clone, Decode, Encode)]
pub enum MessageToPc {}

#[derive(Debug, Clone, Decode, Encode)]
pub enum MessageToPyBadge {
	NewLevel(AvailableCards)
}

