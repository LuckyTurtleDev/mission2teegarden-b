use std::borrow::BorrowMut;

use m3_map::{Orientation, Player};
use macroquad::prelude::*;

use crate::{cards_ev::CarAction, GameState, PlayerState, RotationPoint, tiles::{TEXTURES, GetTexture, Textures}};

impl GameState {
	///update the current state.
	pub async fn update(&mut self) {
		let _player_events = self.input_players.get_events();
        if self.delta_time >= self.movement_time {
            self.delta_time -= self.movement_time;
            self.next_move();
        }
        self.delta_time += get_frame_time();

        

		//use delta time, to avoid that the logic is effected by frame drops
	}

    pub fn update_player_positions(&mut self) {
        match &mut self.game_run {
            None => !todo!(),
            Some(ref mut round) => {
                let player_textures = TEXTURES.get_player_textures();
                for (x, player) in &mut round.level.iter_player().enumerate() {
                    // update player position
                    if round.player_states[x].next_rotation_point != RotationPoint::NoRotation {
                        let offset_x = (round.player_states[x].position.0 - player.position.0) as f32 / (self.movement_time/self.delta_time);
                        let offset_y = (round.player_states[x].position.1 - player.position.1) as f32 / (self.movement_time/self.delta_time);
                        let new_position = (player.position.0 + offset_x.round() as u8, player.position.1 + offset_y.round() as u8);
                        player.position = new_position;
                    } else {
                        todo!()
                    }
                }
            }
        }; 
    }

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
							position: (new_x as u8, new_y as u8),
							orientation: new_values.2,
							next_action: state.card_iter.next().unwrap(),
                            next_rotation_point: new_values.3,
							card_iter: state.card_iter.clone()
						};
						*state = new_state;
					}
				}
			},
		}
	}
}

fn getRelativeXY(player_state: &PlayerState) -> (i8, i8, Orientation, RotationPoint) {
	match &player_state.next_action {
		None => (0, 0, player_state.orientation, RotationPoint::NoRotation),
		Some(car_action) => {
            let mut rotation_points = [RotationPoint::NoRotation, RotationPoint::NoRotation, RotationPoint::NoRotation, RotationPoint::NoRotation];
			let new_orientations = match car_action {
				CarAction::TurnLeft => {
                    rotation_points = [RotationPoint::TopLeft, RotationPoint::BottomRight, RotationPoint::BottomLeft, RotationPoint::TopRight];
                    [
                        Orientation::West,
                        Orientation::East,
                        Orientation::South,   
                        Orientation::North
				    ]
                },
				CarAction::TurnRight => {
                    rotation_points = [RotationPoint::TopRight, RotationPoint::BottomLeft, RotationPoint::BottomRight, RotationPoint::TopLeft];
                    [
                        Orientation::East,
                        Orientation::West,
                        Orientation::North,
                        Orientation::South
				    ]
                },
				CarAction::DriveForward =>
                    [
                        Orientation::North,
                        Orientation::South,
                        Orientation::West,
                        Orientation::East
                    ]
			};
			match player_state.orientation {
				Orientation::North => (-0, -1, new_orientations[0], rotation_points[0]),
				Orientation::South => (0, 1, new_orientations[1], rotation_points[1]),
				Orientation::West => (-1, 0, new_orientations[2], rotation_points[2]),
				Orientation::East => (1, 0, new_orientations[3], rotation_points[3])
			}
		}
	}
}
