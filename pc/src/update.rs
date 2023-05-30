use m3_map::Orientation;


use crate::{GameState, PlayerState, cards_ev::CarAction};

impl GameState {
	///update the current state.
	pub async fn update(&mut self) {
		let _player_events = self.input_players.get_events();
        
		//use delta time, to avoid that the logic is effected by frame drops
	}

	/*fn car_movement(map: &Map, player: &Player, intended_moves: CardStatus) -> CardStatus {
    let start_coordinates = player.start;
    let mut player_XY = player.start;
    let mut cards = intended_moves;
    cards.iter_mut().for_each(|element| {

    });
}*/

	pub fn next_move(&mut self) {
			match &mut self.game_run {
				None => !todo!(),
				Some(ref mut round) => {
					for state in &mut round.player_states {
                        let new_values = getRelativeXY(state);
                        let new_x = state.position.0 as i8 + new_values.0;
                        let new_y = state.position.1 as i8 + new_values.1;
                        if new_x < 0 || new_y < 0 {
                            todo!()
                        } else {
                            let new_state = PlayerState {
                                position: (new_x as u8 , new_y as u8),
                                orientation: new_values.2,
                                next_action: state.card_iter.next().unwrap(),
                                card_iter: state.card_iter.clone()
                            };
                            *state = new_state;
                            //self.game_run.player_states.push(new_state);
                        }
					}
				}
			}
	}
}


fn getRelativeXY(player_state: &PlayerState) ->(i8, i8, Orientation){


    match &player_state.next_action {
        None => (0, 0, player_state.orientation),
        Some(car_action) => {
            let new_orientations = match car_action {
                CarAction::TurnLeft => [Orientation::West, Orientation::East, Orientation::South, Orientation::North],
                CarAction::TurnRight => [Orientation::East, Orientation::West, Orientation::North, Orientation::South],
                CarAction::DriveForward => [Orientation::North, Orientation::South, Orientation::West, Orientation::East]
            };
            match player_state.orientation {
                Orientation::North => (-0, -1, new_orientations[0]),
                Orientation::South => (0, 1, new_orientations[1]),
                Orientation::West => (-1, 0, new_orientations[2]),
                Orientation::East => (1, 0, new_orientations[3])
            }
        }
    }
}