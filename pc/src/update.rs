use crate::evaluate_cards;
use m3_map::Orientation;
use m3_models::{GameOver, NeoPixelColor, ToPcGameEvent, ToPypadeGameEvent};
use macroquad::prelude::*;

use crate::{cards_ev::CarAction, GameState, PlayerState, Rotation};

fn setup_players(events: [Option<Vec<ToPcGameEvent>>; 4], game_state: &mut GameState) {
	if game_state.player_count < events.iter().flatten().count() as u8 {
		if let Some(player) = game_state.input_players.players.iter().flatten().last() {
			game_state.player_count += 1;
			player.send_events(ToPypadeGameEvent::NewLevel(
				game_state.game_run.as_ref().unwrap().level.cards.clone()
			));
			let color = match game_state.player_count {
				1 => NeoPixelColor { r: 10, g: 0, b: 0 },
				2 => NeoPixelColor {
					r: 88,
					g: 76,
					b: 25
				},
				_ => panic!()
			};
			player.send_events(ToPypadeGameEvent::NeoPixelColor(color));
		}
	}
	// get player cards
	for (x, player_events) in events.iter().enumerate() {
		if let Some(player_events) = player_events {
			for event in player_events {
				if let ToPcGameEvent::Solution(solution) = event {
					let cards: Vec<_> = solution
						.into_iter()
						.flatten()
						.map(|f| f.to_owned())
						.collect();
					game_state.game_run.as_mut().unwrap().player_states[x].card_iter =
						Some(evaluate_cards(cards));
				}
			}
		}
	}
	if game_state
		.game_run
		.as_ref()
		.unwrap()
		.player_states
		.iter()
		.filter(|f| f.card_iter.is_some())
		.count() as u8
		== game_state.player_count
		&& game_state.player_count > 0
	{
		game_state.activity = crate::Activity::Drive;
	}
}
impl GameState {
	///update the current state.
	pub(crate) async fn update(&mut self) {
		let events = self.input_players.get_events();
		match &mut self.activity {
			crate::Activity::Select => setup_players(events, self),
			crate::Activity::Drive => {
				//let _player_events = self.input_players.get_events();
				if self.delta_time >= self.movement_time {
					self.delta_time -= self.movement_time;

					self.next_move();
				}
				self.delta_time += get_frame_time();
			}
		}
	}

	pub(crate) fn next_move(&mut self) {
		if let Some(ref mut game_run) = self.game_run {
			// update player position
			for (x, player) in game_run.level.iter_mut_player().enumerate() {
				player.position = game_run.player_states[x].position;
				player.orientation = game_run.player_states[x].orientation;
			}
			//update next state
			for (x, state) in &mut game_run.player_states.iter_mut().enumerate() {
				let new_values = get_relative_xy(state);
				let new_x = state.position.0 as i8 + new_values.0;
				let new_y = state.position.1 as i8 + new_values.1;

				if new_y < 0
					|| new_x < 0 || new_x >= game_run.level.width as i8
					|| new_y >= game_run.level.height as i8
				{
					if self.input_players.players[x].as_ref().is_some() {
						debug!("Durch Update GameOver");
						self.input_players.players[x].as_ref().unwrap().send_events(
							ToPypadeGameEvent::GameOver(GameOver::DriveAway)
						);
					}
				} else {
					let new_state = PlayerState {
						position: (new_x as u8, new_y as u8),
						orientation: new_values.2,
						next_action: match &mut state.card_iter {
							Some(iter) => iter.next().unwrap(),
							None => Some(CarAction::DriveForward)
						},
						rotation: new_values.3,
						card_iter: state.card_iter.clone()
					};
					*state = new_state;
				}
			}
			// check for collisions with other players
			for x in 0..3 {
				for y in x + 1..4 {
					if game_run.player_states[x].position
						== game_run.player_states[y].position
					{
						debug!(
							"Player Position: {:?}, {:?}, {:?}, {:?}",
							game_run.player_states[0].position,
							game_run.player_states[1].position,
							game_run.player_states[2].position,
							game_run.player_states[3].position
						);
						if self.input_players.players[x].as_ref().is_some() {
							self.input_players.players[x].as_ref().unwrap().send_events(
								ToPypadeGameEvent::GameOver(GameOver::Crash)
							);
						}
						if self.input_players.players[y].as_ref().is_some() {
							self.input_players.players[y].as_ref().unwrap().send_events(
								ToPypadeGameEvent::GameOver(GameOver::Crash)
							);
						}
					}
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
