use m3_models::CardIter;
use m3_map::Orientation;

use crate::GameState;

impl GameState {
	///update the current state.
	pub async fn update(&mut self) {
		let _player_events = self.players.get_events();
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
				Some(mut run) => {
					for (i, player) in run.level.iter_mut_player().enumerate() {
						run.player_states.get(i).position = getRelativeXY(run.player_states.get(i).run.player_states.get(i).orientation);
					}
				}
			}
		}
	}

}


fn getRelativeXY<CarAction>(car_action: Option<CarAction>, orientation: Orientation) ->(i8, u8, Orientation){
    match car_action {
        None => (0, 0, orientation),
        TurnLeft => {
            match orientation {
                North => (-1, -1, Orientation::West),
                South => (1, 1, Orientation::East),
                West => (-1, 1, Orientation::South),
                East => (1, -1, Orientation::North)
            }
        },
        TurnRight => {
            match orientation {
                North => (-1, 1, Orientation::East),
                South => (-1, 1, Orientation::West),
                West => (-1, -1, Orientation::North),
                East => (1, 1, Orientation::South)
            }
        },
        DriveForward => {
            match orientation {
                North => (0, -1, Orientation::North),
                South => (0, Orientation::South),
                West => (-1, Orientation::West),
                East => (1, Orientation::East)
            }
        }
    }
}
