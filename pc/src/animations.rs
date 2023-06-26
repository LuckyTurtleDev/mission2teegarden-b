use crate::GameState;
use macroquad::prelude::*;
use macroquad_particles::{AtlasConfig, BlendMode, Emitter, EmitterConfig};

impl GameState {
	pub(crate) async fn load_fire_emitter(&mut self) {
		let texture = load_texture("assets/img/Animations/crash_fire.png")
			.await
			.unwrap();
		let emitter = Emitter::new(EmitterConfig {
			local_coords: false,
			texture: Some(texture),
			lifetime: 0.5,
			amount: 3,
			atlas: Some(AtlasConfig::new(4, 4, 12..)),
			size: 20.0,
			blend_mode: BlendMode::Additive,
			..Default::default()
		});
		self.animation_emitter = Some(emitter);
	}
}
