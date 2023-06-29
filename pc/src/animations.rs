use crate::GameState;
use macroquad::prelude::*;
use macroquad_particles::{AtlasConfig, BlendMode, Emitter, EmitterConfig};

impl GameState {
	pub(crate) async fn load_fire_emitter(&mut self) {
		let texture = load_texture("assets/img/Animations/crash_fire_2.png")
			.await
			.unwrap();
		let size = ((screen_width()
			/ self.game_run.as_ref().unwrap().level.width as f32)
			.min(screen_height() / self.game_run.as_ref().unwrap().level.height as f32))
			/ 3.0;
		debug!("screen_height: {:#?}", screen_height());
		let emitter = Emitter::new(EmitterConfig {
			local_coords: false,
			texture: Some(texture),
			lifetime: 2.0,
			lifetime_randomness: 0.7,
			explosiveness: 0.95,
			initial_direction_spread: 0.5,
			amount: 15,
			atlas: Some(AtlasConfig::new(4, 4, 0..8)),
			size,
			blend_mode: BlendMode::Additive,
			..Default::default()
		});
		self.animation_emitter = Some(emitter);
	}
}
