use m3_map::Orientation;
use m3_models::{GameOver, ToPypadeGameEvent};
use macroquad::prelude::*;

use crate::{cards_ev::CarAction, GameState, PlayerState, Rotation};

impl GameState {
	///update the current state.
	pub async fn update(&mut self) {
		//let _player_events = self.input_players.get_events();
		if self.delta_time >= self.movement_time {
			self.delta_time -= self.movement_time;

			self.next_move();
		}
		self.delta_time += get_frame_time();
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
				let mut array: [(i8, i8); 4] = [(-1, -1), (-1, -1), (-1, -1), (-1, -1)];
				for (x, state) in &mut game_run.player_states.iter_mut().enumerate() {
					let new_values = get_relative_xy(state);
					let new_x = state.position.0 as i8 + new_values.0;
					let new_y = state.position.1 as i8 + new_values.1;
					if new_x < 0 || new_y < 0 {
						if self.input_players.players[x].as_ref().is_some() {
							debug!("Spieler {}", x);
							self.input_players.players[x].as_ref().unwrap().send_events(
								ToPypadeGameEvent::GameOver(GameOver::DriveAway)
							);
						}
					} else if array.contains(&(new_x, new_y)) {
						if self.input_players.players[x].as_ref().is_some() {
							self.input_players.players[x].as_ref().unwrap().send_events(
								ToPypadeGameEvent::GameOver(GameOver::Crash)
							);
						}
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
					array[x] = (new_x, new_y);
				}
			}
		}
	}
}

fn get_relative_xy(player_state: &PlayerState) -> (i8, i8, Orientation, Rotation) {
	let mut rotation = Rotation::NoRotation;
	match &player_state.next_action {
		None => (0, 0, player_state.orientation, rotation),
		Some(car_action) => {
			let mut relative_pos = (0, 0);
			let new_orientations = match car_action {
				CarAction::RotateLeft => {
					rotation = Rotation::RotateLeft;
					[
						Orientation::West,
						Orientation::East,
						Orientation::South,
						Orientation::North
					]
				},
				CarAction::RotateRight => {
					rotation = Rotation::RotateRight;
					[
						Orientation::East,
						Orientation::West,
						Orientation::North,
						Orientation::South
					]
				},
				CarAction::DriveForward => {
					relative_pos = match player_state.orientation {
						Orientation::North => (0, -1),
						Orientation::South => (0, 1),
						Orientation::West => (-1, 0),
						Orientation::East => (1, 0)
					};
					[
						Orientation::North,
						Orientation::South,
						Orientation::West,
						Orientation::East
					]
				}
			};
			match player_state.orientation {
				Orientation::North => (
					relative_pos.0,
					relative_pos.1,
					new_orientations[0],
					rotation
				),
				Orientation::South => (
					relative_pos.0,
					relative_pos.1,
					new_orientations[1],
					rotation
				),
				Orientation::West => (
					relative_pos.0,
					relative_pos.1,
					new_orientations[2],
					rotation
				),
				Orientation::East => (
					relative_pos.0,
					relative_pos.1,
					new_orientations[3],
					rotation
				)
			}
		}
	}
}
