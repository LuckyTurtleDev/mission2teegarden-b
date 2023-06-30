use mission2teegarden_b_models::Card;

#[derive(Debug, PartialEq)]
pub(crate) enum CarAction {
	RotateLeft,
	RotateRight,
	DriveForward
}

#[derive(Clone, Debug)]
pub(crate) struct CardIter {
	/// Position of card in vector
	card_pos: usize,
	/// Relative y-position to former position
	wait_counter: u8,
	driving: bool,
	cards: Vec<Card>
}

impl Default for CardIter {
	fn default() -> Self {
		Self {
			card_pos: 0,
			wait_counter: 0,
			driving: true,
			cards: Vec::with_capacity(0)
		}
	}
}

impl Iterator for CardIter {
	type Item = (Option<usize>, Option<CarAction>);

	fn next(&mut self) -> Option<Self::Item> {
		let card_pos = self.card_pos;
		Some(match self.cards.get(self.card_pos) {
			None => {
				if self.driving {
					(None, Some(CarAction::DriveForward))
				} else {
					(None, None)
				}
			},
			Some(card) => {
				let car_action = match card {
					Card::Left => {
						self.card_pos += 1;
						Some(CarAction::RotateLeft)
					},
					Card::Right => {
						self.card_pos += 1;
						Some(CarAction::RotateRight)
					},
					Card::Wait(i) => {
						if self.wait_counter < (*i) - 1 {
							self.wait_counter += 1;
							if self.driving {
								Some(CarAction::DriveForward)
							} else {
								None
							}
						} else {
							self.wait_counter = 0;
							self.card_pos += 1;
							if self.driving {
								Some(CarAction::DriveForward)
							} else {
								None
							}
						}
					},
					Card::MotorOn => {
						self.card_pos += 1;
						self.driving = true;
						None
					},
					Card::MotorOff => {
						self.card_pos += 1;
						self.driving = false;
						None
					}
				};
				(Some(card_pos), car_action)
			}
		})
	}
}

pub(crate) fn evaluate_cards(cards: Vec<Card>) -> CardIter {
	CardIter {
		cards,
		..Default::default()
	}
}

#[cfg(test)]
mod tests {
	use mission2teegarden_b_models::Card::{Left, MotorOff, MotorOn, Wait};

	use crate::cards_ev::{evaluate_cards, CarAction::*};
	#[test]
	fn test_card_evaluation() {
		let cards = vec![MotorOn, Wait(3), Left, Wait(2), MotorOff];
		let card_iter = evaluate_cards(cards).take(6);
		let correct_actions = vec![
			(Some(0), None),
			(Some(1), Some(DriveForward)),
			(Some(1), Some(DriveForward)),
			(Some(1), Some(DriveForward)),
			(Some(2), Some(RotateLeft)),
			(Some(3), Some(DriveForward)),
			(Some(3), Some(DriveForward)),
			(Some(4), None),
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
