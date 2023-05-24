use m3_models::CardIter;
use m3_map::Orientation;

use crate::{GameState, PlayerState};

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

	pub fn play_game(&mut self) {
		loop {
			match self.game_run {
				None => !todo!(),
				Some(mut round) => {
					for (i, state) in round.player_states {
                        let new_state = getRelativeXY(state);
						//round.player_states.get(i).unwrap().position = ;
					}
				}
			}
		}
	}

}


fn getRelativeXY<CarAction>(player_state: PlayerState) ->(i8, i8, Orientation){
    match player_state.next_action {
        None => (0, 0, player_state.orientation),
        TurnLeft => {
            match player_state.orientation {
                North => (-1, -1, Orientation::West),
                South => (1, 1, Orientation::East),
                West => (-1, 1, Orientation::South),
                East => (1, -1, Orientation::North)
            }
        },
        TurnRight => {
            match player_state.orientation {
                North => (-1, 1, Orientation::East),
                South => (-1, 1, Orientation::West),
                West => (-1, -1, Orientation::North),
                East => (1, 1, Orientation::South)
            }
        },
        DriveForward => {
            match player_state.orientation {
                North => (0, -1, Orientation::North),
                South => (0, Orientation::South),
                West => (-1, Orientation::West),
                East => (1, Orientation::East)
            }
        }
    }
}
