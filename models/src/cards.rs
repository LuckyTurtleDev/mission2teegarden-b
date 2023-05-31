use bincode::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumCount, EnumIter};

#[derive(AsRefStr, Clone, Debug, Decode, Encode, EnumCount, EnumIter, PartialEq)]
pub enum Card {
	/// Turn Left
	Left,
	/// Turn Right
	Right,
	/// Keep doing current action
	Wait(u8),
	/// Starts driving foward
	MotorOn,
	/// Stops driving
	MotorOff
}

impl Card {
	/// return embeded sprite
	pub fn picture() {
		unimplemented!()
	}
	/// return the image path
	pub fn path() {
		unimplemented!()
	}
}

/// count of cards, witch are avaibale for the player
#[derive(Clone, Debug, Decode, Default, Encode, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AvailableCards {
	#[cfg_attr(feature = "serde", serde(default))]
	pub left: u8,
	#[cfg_attr(feature = "serde", serde(default))]
	pub right: u8,
	#[cfg_attr(feature = "serde", serde(default))]
	pub wait: u8,
	#[cfg_attr(feature = "serde", serde(default))]
	pub motor_on: u8,
	#[cfg_attr(feature = "serde", serde(default))]
	pub motor_off: u8
}

impl AvailableCards {
	///return how many cards are avaible from the requested variant `card`
	pub fn card_count(&self, card: &Card) -> u8 {
		match card {
			Card::Left => self.left,
			Card::Right => self.right,
			Card::Wait(_i) => self.wait,
			Card::MotorOn => self.motor_on,
			Card::MotorOff => self.motor_off
		}
	}

	pub fn card_count_mut(&mut self, card: &Card) -> &mut u8 {
		match card {
			Card::Left => &mut self.left,
			Card::Right => &mut self.right,
			Card::Wait(_i) => &mut self.wait,
			Card::MotorOn => &mut self.motor_on,
			Card::MotorOff => &mut self.motor_off
		}
	}

	pub fn set_card_count_mut(&mut self, card: &Card, count: u8) {
		match card {
			Card::Left => self.left = count,
			Card::Right => self.right = count,
			Card::Wait(_i) => self.wait = count,
			Card::MotorOn => self.motor_on = count,
			Card::MotorOff => self.motor_off = count
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Card;
	#[test]
	fn as_ref_str() {
		assert_eq!("Left", Card::Left.as_ref());
	}
}
