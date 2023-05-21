use crate::GameState;

impl GameState {
	///update the current state.
	pub async fn update(&mut self) {
		let _player_events = self.players.get_events();
		//use delta time, to avoid that the logic is effected by frame drops
	}
}
