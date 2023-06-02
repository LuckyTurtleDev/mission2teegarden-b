use std::borrow::BorrowMut;

use m3_map::{Orientation, Player};
use macroquad::prelude::*;

use crate::{
	cards_ev::CarAction,
	tiles::{GetTexture, Textures, TEXTURES},
	GameState, PlayerState, Rotation, RotationPoint
};

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

	pub fn next_move(&mut self) {
		match &mut self.game_run {
			None => !todo!(),
			Some(ref mut game_run) => {
				// update player position
				for (x, player) in &mut game_run.level.iter_mut_player().enumerate() {
					player.position = game_run.player_states[x].position;
					player.orientation = game_run.player_states[x].orientation;
				}

				//update next state
				for state in &mut game_run.player_states {
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
							rotation: new_values.3,
							card_iter: state.card_iter.clone()
						};
						*state = new_state;
					}
				}
			}
		}
	}
}

fn getRelativeXY(player_state: &PlayerState) -> (i8, i8, Orientation, Rotation) {
	match &player_state.next_action {
		None => (0, 0, player_state.orientation, Rotation {
			pivot: RotationPoint::NoRotation,
			sign: 1
		}),
		Some(car_action) => {
			let mut rotation_pivots = [
				RotationPoint::NoRotation,
				RotationPoint::NoRotation,
				RotationPoint::NoRotation,
				RotationPoint::NoRotation
			];
			let mut rotation_sign = 1;
			let new_orientations = match car_action {
				CarAction::TurnLeft => {
					rotation_pivots = [
						RotationPoint::TopLeft,
						RotationPoint::BottomRight,
						RotationPoint::BottomLeft,
						RotationPoint::TopRight
					];
					rotation_sign = -1;
					[
						Orientation::West,
						Orientation::East,
						Orientation::South,
						Orientation::North
					]
				},
				CarAction::TurnRight => {
					rotation_pivots = [
						RotationPoint::TopRight,
						RotationPoint::BottomLeft,
						RotationPoint::BottomRight,
						RotationPoint::TopLeft
					];
					[
						Orientation::East,
						Orientation::West,
						Orientation::North,
						Orientation::South
					]
				},
				CarAction::DriveForward => [
					Orientation::North,
					Orientation::South,
					Orientation::West,
					Orientation::East
				]
			};
			match player_state.orientation {
				Orientation::North => (0, -1, new_orientations[0], Rotation {
					pivot: rotation_pivots[0],
					sign: rotation_sign
				}),
				Orientation::South => (0, 1, new_orientations[1], Rotation {
					pivot: rotation_pivots[1],
					sign: rotation_sign
				}),
				Orientation::West => (-1, 0, new_orientations[2], Rotation {
					pivot: rotation_pivots[2],
					sign: rotation_sign
				}),
				Orientation::East => (1, 0, new_orientations[3], Rotation {
					pivot: rotation_pivots[3],
					sign: rotation_sign
				})
			}
		}
	}
}
