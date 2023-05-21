use crate::{GameState, TEXTURES};

use crate::tiles::GetTexture;
use log::{debug, info};
use m3_macro::include_map;
use m3_map::Map;
use macroquad::{math::Vec2, prelude::*, window, Window};
use my_env_logger_style::TimestampPrecision;
use once_cell::sync::Lazy;

impl GameState {
	///update the current state.
	pub async fn update(&mut self) {
		let player_events = self.players.get_events();
		//use delta time, to avoid that the logic is effected by frame drops
	}
}
