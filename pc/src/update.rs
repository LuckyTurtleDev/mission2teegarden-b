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
		loop {
			match &self.game_run {
				None => !todo!(),
				Some(round) => {
					for state in round.player_states {
                        let new_state = getRelativeXY(state);
						state.position.0 += new_state.0;
                        state.position.1 += new_state.1;
                        state.orientation = new_state.2;
                        state.next_action = state.card_iter.next();
					}
				}
			}
		}
	}
}


fn getRelativeXY(player_state: PlayerState) ->(i8, i8, Orientation){


    match player_state.next_action {
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