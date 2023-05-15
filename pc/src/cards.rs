use m3_models::Card;
use m3_models::Card::*;



pub enum CarAction {
    TurnLeft,
    TurnRight,
    DriveForward,
    Nothing
}
pub struct CardChanges<'a> {
    /// Position of card in vector
    card_pos: usize,
    /// Relative y-position to former position
    wait_counter: u8,
    driving: bool,
    cards: &'a Vec<Card>
}


impl<'a> Iterator for CardChanges<'a> {
    type Item = Option<CarAction>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cards.get(self.card_pos) {
            None => {
                if self.driving {
                   Some(Some(CarAction::DriveForward))
                } else {
                    Some(Some(CarAction::Nothing))
                }
            }
            Some(card) => {
                match card {
                    Card::Left => {
                        self.card_pos += 1;
                        self.driving = true;
                        Some(Some(CarAction::TurnLeft))
                    }
                    Right => {
                        self.card_pos += 1;
                        self.driving = true;
                        Some(Some(CarAction::TurnRight))
                    }
                    Wait(i) => {
                        if self.wait_counter < *i {
                            self.wait_counter += 1;
                            if self.driving {
                                Some(Some(CarAction::DriveForward))
                            } else {
                                Some(Some(CarAction::Nothing))
                            }
                        } else {
                            Some(Some(CarAction::Nothing))
                        }
                    }
                    MotorOn => {
                        self.driving = true;
                        Some(Some(CarAction::Nothing))
                    }
                    MotorOff => {
                        self.driving = false;
                        Some(Some(CarAction::Nothing))
                    }
                }
            }
        }
    }
}

pub fn evaluateCards(cards:&Vec<Card>) -> CardChanges{
    CardChanges { card_pos: 0, wait_counter: 0, driving: true, cards: cards }
}