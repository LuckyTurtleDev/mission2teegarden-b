use crate::{
	cards_ev::CarAction, evaluate_cards, Activity, GameRun, GameState, Map, Phase,
	PlayerState, Rotation, LEVELS
};
use m3_map::Orientation;
use m3_models::{
	AvailableCards, GameOver, Key, NeoPixelColor, ToPcGameEvent, ToPypadeGameEvent
};
use macroquad::prelude::*;

fn reset_button_pressed(events: &[Option<Vec<ToPcGameEvent>>; 4]) -> bool {
	for player_events in events.iter().flatten() {
		for event in player_events {
			if let ToPcGameEvent::KeyPressed(key) = event {
				return *key == Key::Select;
			}
		}
	}
	false
}

pub(crate) fn activate_players(
	game_state: &mut GameState,
	to_pybadge_game_event: ToPypadeGameEvent
) {
	for player in game_state.input_players.players.iter().flatten() {
		player.send_events(to_pybadge_game_event.clone());
	}
}

pub(crate) fn init_level(game_state: &mut GameState) {
	let mut level = Map::from_string(LEVELS[game_state.level_num]).unwrap();
	level.cards = AvailableCards {
		left: 3,
		right: 3,
		motor_on: 2,
		motor_off: 2,
		wait: 4
	};
	let player_states = level
		.iter_player()
		.map(|f| PlayerState {
			position: f.position,
			orientation: f.orientation,
			next_action: None,
			rotation: Rotation::NoRotation,
			finished: false,
			crashed: false,
			out_of_map: false,
			solution: None
		})
		.collect();
	let game_run = GameRun {
		level,
		player_states,
		player_finished_level: 0
	};
	game_state.game_run = Some(game_run);
	game_state.delta_time = 0.0;
}

pub(crate) async fn setup_players(game_state: &mut GameState) {
	let events = game_state.input_players.get_events();
	if game_state.player_count < events.iter().flatten().count() as u8 {
		if let Some(player) = game_state.input_players.players.iter().flatten().last() {
			game_state.player_count += 1;
			player.send_events(ToPypadeGameEvent::NewLevel(
				game_state.game_run.as_ref().unwrap().level.cards.clone()
			));
			let color = match game_state.player_count {
				1 => NeoPixelColor { r: 20, g: 20, b: 0 },
				2 => NeoPixelColor { r: 38, g: 2, b: 0 },
				3 => NeoPixelColor { r: 2, g: 2, b: 16 },
				4 => NeoPixelColor { r: 20, g: 0, b: 20 },
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
					let cards: Vec<_> =
						solution.iter().flatten().map(|f| f.to_owned()).collect();
					game_state.game_run.as_mut().unwrap().player_states[x].solution =
						Some(evaluate_cards(cards));
				}
			}
		}
	}
	// check if all player has submit an solution.
	if game_state
		.game_run
		.as_ref()
		.unwrap()
		.player_states
		.iter()
		.filter(|f| f.solution.is_some())
		.count() as u8
		== game_state.player_count
		&& game_state.player_count > 0
	{
		game_state.activity = crate::Activity::GameRound(Phase::Drive);
		for player in game_state.input_players.players.iter().flatten() {
			player.send_events(ToPypadeGameEvent::Driving);
		}
	}
}

impl GameState {
	/// update the current state.
	pub(crate) async fn update(&mut self) {
		let events = self.input_players.get_events();
		if reset_button_pressed(&events) {
			init_level(self);
			activate_players(self, ToPypadeGameEvent::Retry);
			self.activity = Activity::GameRound(Phase::Select);
		} else {
			if self.delta_time >= self.movement_time {
				if self.game_run.as_ref().unwrap().player_finished_level
					== self.player_count
				{
					self.activity = Activity::GameRound(Phase::Finish);
				}
				self.delta_time -= self.movement_time;
				self.next_move();
			}
			self.delta_time += get_frame_time();
		}
	}

	/// calculate next moves
	pub(crate) fn next_move(&mut self) {
		if let Some(ref mut game_run) = self.game_run {
			let crashed_player_count = game_run
				.player_states
				.iter()
				.filter(|player| player.crashed)
				.count();
			// update player positions
			let global_goal = game_run.level.global_goal;
			for (player, player_state) in game_run
				.level
				.iter_mut_player()
				.zip(game_run.player_states.iter_mut())
			{
				player.position = player_state.position;
				player.orientation = player_state.orientation;
				if let Some(global_goal) = global_goal {
					if player.position.0 == global_goal.0
						&& player.position.1 == global_goal.1
					{
						player_state.finished = true;
						game_run.player_finished_level += 1;
					}
				} else if let Some(goal) = player.goal {
					if player.position.0 == goal.0 && player.position.1 == goal.1 {
						player_state.finished = true;
						game_run.player_finished_level += 1;
					}
				}
			}
			//update next state
			for (state, player) in game_run
				.player_states
				.iter_mut()
				.zip(self.input_players.players.iter())
			{
				if state.out_of_map {
					state.finished = true;
					if let Some(ref player) = player {
						player.send_events(ToPypadeGameEvent::GameOver(
							GameOver::DriveAway
						));
					}
				}
				if !state.finished && !state.crashed {
					let new_values = get_relative_xy(state);
					let new_x = state.position.0 as i8 + new_values.0;
					let new_y = state.position.1 as i8 + new_values.1;

					if (new_y < 0
						|| new_x < 0 || new_x >= game_run.level.width as i8
						|| new_y >= game_run.level.height as i8)
						&& !state.finished
					{
						state.out_of_map = true;
					}
					if !game_run.level.passable(new_x as u8, new_y as u8)
						&& !state.crashed && player.is_some()
					{
						player
							.as_ref()
							.unwrap()
							.send_events(ToPypadeGameEvent::GameOver(GameOver::Crash));
						state.crashed = true;
					} else {
						state.position = (new_x as u8, new_y as u8);
						state.orientation = new_values.2;
						state.rotation = new_values.3;
						state.next_action = match &mut state.solution {
							Some(iter) => {
								let (index, action) = iter.next().unwrap();

								if let Some(ref player) = player {
									player.send_events(
										ToPypadeGameEvent::CurrentCardIndex(
											index.map(|f| f as u8)
										)
									);
								}

								action
							},
							None => Some(CarAction::DriveForward)
						};
					}
				}
			}
			// check for collisions with other players
			for x in 0..3 {
				for y in 1..4 {
					if y <= x {
						continue;
					}
					if self.input_players.players[x].as_ref().is_some()
						&& self.input_players.players[y].as_ref().is_some()
						&& (!game_run.player_states[x].finished
							&& !game_run.player_states[y].finished)
						&& (game_run.level.iter_player().nth(x).unwrap().position
							== game_run.level.iter_player().nth(y).unwrap().position
							|| game_run.player_states[x].position
								== game_run.level.iter_player().nth(y).unwrap().position
							|| game_run.player_states[y].position
								== game_run.level.iter_player().nth(x).unwrap().position)
					{
						self.input_players.players[x]
							.as_ref()
							.unwrap()
							.send_events(ToPypadeGameEvent::GameOver(GameOver::Crash));
						self.input_players.players[y]
							.as_ref()
							.unwrap()
							.send_events(ToPypadeGameEvent::GameOver(GameOver::Crash));
						game_run.player_states[x].crashed = true;
						game_run.player_states[y].crashed = true;
						game_run.player_states[x].position =
							game_run.level.iter_player().nth(x).unwrap().position;
						game_run.player_states[y].position =
							game_run.level.iter_player().nth(y).unwrap().position;
					}
				}
			}
			if crashed_player_count
				!= game_run
					.player_states
					.iter()
					.filter(|player| player.crashed)
					.count()
			{
				self.sound_player.play_crash();
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
