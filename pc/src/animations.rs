use crate::GameState;
use macroquad::prelude::*;
use macroquad_particles::{AtlasConfig, BlendMode, Emitter, EmitterConfig};

impl GameState {
	pub(crate) async fn load_fire_emitter(&mut self) {
		let texture = load_texture("assets/img/Animations/crash_fire_2.png")
			.await
			.unwrap();
		let emitter = Emitter::new(EmitterConfig {
			local_coords: false,
			texture: Some(texture),
			lifetime: 0.8,
			lifetime_randomness: 0.7,
			explosiveness: 0.95,
			initial_direction_spread: 0.5,
			amount: 20,
			atlas: Some(AtlasConfig::new(4, 4, 0..8)),
			size: 15.0,
			blend_mode: BlendMode::Additive,
			..Default::default()
		});
		self.animation_emitter = Some(emitter);
	}
}
