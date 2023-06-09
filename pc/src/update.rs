use crate::{
	cards_ev::CarAction, evaluate_cards, GameState, Map, PlayerState, Rotation, LEVELS,
	LEVEL_NR
};
use m3_map::Orientation;
use m3_models::{GameOver, Key, NeoPixelColor, ToPcGameEvent, ToPypadeGameEvent};
use macroquad::prelude::*;

fn wants_reset(events: [Option<Vec<ToPcGameEvent>>; 4]) -> bool {
	for player_events in events.into_iter().flatten() {
		for event in player_events {
			if let ToPcGameEvent::KeyPressed(key) = event {
				return key == Key::Select;
			}
		}
	}
	false
}

fn reset_level(game_state: &mut GameState) {
	let level = Map::from_string(LEVELS[LEVEL_NR]).unwrap();
	for (x, player) in game_state
		.input_players
		.players
		.iter()
		.flatten()
		.enumerate()
	{
		player.send_events(ToPypadeGameEvent::Retry);
		game_state.game_run.as_mut().unwrap().player_states[x].position =
			level.iter_player().next().unwrap().position;
	}
	for (x, (x, player)) in game_state
		.game_run
		.as_mut()
		.unwrap()
		.level
		.iter_mut_player()
		.enumerate()
		.enumerate()
	{
		player.position = level.iter_player().nth(x).unwrap().position;
		player.orientation = level.iter_player().nth(x).unwrap().orientation;
	}
	let player_states = level
		.iter_player()
		.map(|f| PlayerState {
			position: f.position,
			orientation: f.orientation,
			next_action: None,
			rotation: Rotation::NoRotation,
			finished: false,
			crashed: false,
			card_iter: None
		})
		.collect();
	game_state.game_run.as_mut().unwrap().player_states = player_states;
	game_state.delta_time = 0.0;
	game_state.activity = crate::Activity::Select;
}

fn setup_players(events: [Option<Vec<ToPcGameEvent>>; 4], game_state: &mut GameState) {
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
				if wants_reset(events) {
					reset_level(self);
				} else {
					if self.delta_time >= self.movement_time {
						self.delta_time -= self.movement_time;

						self.next_move();
					}
					self.delta_time += get_frame_time();
				}
			},
		}
	}

	pub(crate) fn next_move(&mut self) {
		if let Some(ref mut game_run) = self.game_run {
			// update player positions
			let global_goal = game_run.level.global_goal;
			for (x, player) in game_run.level.iter_mut_player().enumerate() {
				player.position = game_run.player_states[x].position;
				player.orientation = game_run.player_states[x].orientation;

				if let Some(global_goal) = global_goal {
					if player.position.0 == global_goal.0
						&& player.position.1 == global_goal.1
					{
						game_run.player_states[x].finished = true;
					}
				} else if let Some(goal) = player.goal {
					if player.position.0 == goal.0 && player.position.1 == goal.1 {
						game_run.player_states[x].finished = true;
					}
				}
			}
			//update next state
			for (x, state) in &mut game_run.player_states.iter_mut().enumerate() {
				if !state.finished && !state.crashed {
					let new_values = get_relative_xy(state);
					let new_x = state.position.0 as i8 + new_values.0;
					let new_y = state.position.1 as i8 + new_values.1;

					if (new_y < 0
						|| new_x < 0 || new_x >= game_run.level.width as i8
						|| new_y >= game_run.level.height as i8)
						&& !state.finished
					{
						if self.input_players.players[x].as_ref().is_some() {
							self.input_players.players[x].as_ref().unwrap().send_events(
								ToPypadeGameEvent::GameOver(GameOver::DriveAway)
							);
						}
					} else if !game_run.level.passable(new_x as u8, new_y as u8)
						&& !state.crashed && self.input_players.players[x]
						.as_ref()
						.is_some()
					{
						self.input_players.players[x]
							.as_ref()
							.unwrap()
							.send_events(ToPypadeGameEvent::GameOver(GameOver::Crash));
						state.crashed = true;
					} else {
						let new_state = PlayerState {
							position: (new_x as u8, new_y as u8),
							orientation: new_values.2,
							next_action: match &mut state.card_iter {
								Some(iter) => iter.next().unwrap(),
								None => Some(CarAction::DriveForward)
							},
							rotation: new_values.3,
							finished: state.finished,
							crashed: state.crashed,
							card_iter: state.card_iter.clone()
						};
						*state = new_state;
					}
				}
			}
			// check for collisions with other players
			for x in 0..3 {
				for y in x + 1..4 {
					if self.input_players.players[x].as_ref().is_some()
						&& self.input_players.players[y].as_ref().is_some()
						&& (!game_run.player_states[x].finished
							&& !game_run.player_states[y].finished)
						&& (game_run.player_states[x].position
							== game_run.player_states[y].position
							|| game_run.player_states[x].position
								== game_run.level.iter_player().nth(y).unwrap().position)
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
