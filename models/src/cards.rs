use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};

#[derive(AsRefStr, Clone, Copy, Debug, EnumIter)]
pub enum Card {
	Left,
	Right
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
#[derive(Clone, Debug, Decode, Default, Deserialize, Encode, PartialEq, Serialize)]
pub struct AvailableCards {
	pub left: u8,
	pub right: u8
}

impl AvailableCards {
	///return how many cards are avaible from the requested variant `card`
	pub fn card_count(&self, card: Card) -> u8 {
		match card {
			Card::Left => self.left,
			Card::Right => self.right
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
