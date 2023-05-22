use m3_models::Card;

#[derive(Debug, PartialEq)]
pub enum CarAction {
	TurnLeft,
	TurnRight,
	DriveForward
}

#[derive(Clone, Debug)]
pub struct CardIter<'a> {
	/// Position of card in vector
	card_pos: usize,
	/// Relative y-position to former position
	wait_counter: u8,
	driving: bool,
	cards: &'a Vec<Card>
}

impl<'a> Iterator for CardIter<'a> {
	type Item = Option<CarAction>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.cards.get(self.card_pos) {
			None => {
				if self.driving {
					Some(Some(CarAction::DriveForward))
				} else {
					Some(None)
				}
			},
			Some(card) => match card {
				Card::Left => {
					self.card_pos += 1;
					self.driving = true;
					Some(Some(CarAction::TurnLeft))
				},
				Card::Right => {
					self.card_pos += 1;
					self.driving = true;
					Some(Some(CarAction::TurnRight))
				},
				Card::Wait(i) => {
					if self.wait_counter < (*i) - 1 {
						self.wait_counter += 1;
						if self.driving {
							Some(Some(CarAction::DriveForward))
						} else {
							Some(None)
						}
					} else {
						self.wait_counter = 0;
						self.card_pos += 1;
						Some(Some(CarAction::DriveForward))
					}
				},
				Card::MotorOn => {
					self.card_pos += 1;
					self.driving = true;
					Some(None)
				},
				Card::MotorOff => {
					self.card_pos += 1;
					self.driving = false;
					Some(None)
				}
			}
		}
	}
}

pub fn evaluate_cards(cards: &Vec<Card>) -> CardIter {
	CardIter {
		card_pos: 0,
		wait_counter: 0,
		driving: true,
		cards
	}
}

#[cfg(test)]
mod tests {
	use m3_models::Card::{Left, MotorOff, MotorOn, Wait};

	use crate::cards::{evaluate_cards, CarAction::*};
	#[test]
	fn test_card_evaluation() {
		let cards = vec![MotorOn, Wait(3), Left, Wait(2), MotorOff];
		let card_iter = evaluate_cards(&cards).take(6);
		let correct_actions = vec![
			None,
			Some(DriveForward),
			Some(DriveForward),
			Some(DriveForward),
			Some(TurnLeft),
			Some(DriveForward),
			Some(DriveForward),
			None,
		];
		for (i, card) in card_iter.enumerate() {
			assert!(
				card == *(correct_actions.get(i).unwrap()),
				"Action: `{:?}`, Solution: `{:?}`",
				card,
				*(correct_actions.get(i).unwrap())
			);
		}
	}
}
