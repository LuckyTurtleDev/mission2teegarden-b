use tetra::{
	graphics,
	graphics::{Color, Texture},
	Context, ContextBuilder, State
};
type Vec2 = vek::vec::repr_c::vec2::Vec2<f32>;
use tetra::{graphics::DrawParams, time::get_delta_time};

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

//https://tetra.seventeencups.net/tutorial

struct GameState {
	grass_texture: Texture,
	grass_postion: Vec2,
	grass_rotation: f32
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let grass_texture = Texture::new(ctx, "./assets/img/map-tiles/grass.png")?;
		Ok(GameState {
			grass_texture,
			grass_postion: Vec2::default(),
			grass_rotation: 0.0
		})
	}
}

impl State for GameState {
	//draw the current state
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

		//moving sprite
		self.grass_texture.draw(ctx, self.grass_postion);

		//rotating sprite
		self.grass_texture.draw(
			ctx,
			DrawParams::new()
				.origin(Vec2::new(
					(self.grass_texture.width() / 2) as f32,
					(self.grass_texture.height() / 2) as f32
				)) //set origion to center, to rotate at the middle (this does also effect postion())
				.rotation(self.grass_rotation)
				.position(Vec2::new(100.0, 100.0))
		);

		//see https://docs.rs/tetra/latest/tetra/graphics/struct.DrawParams.html
		Ok(())
	}

	//update the current state.
	//is called 60 time pro seconds (alsong framerated does not drop)
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<()> {
		//use delta time, to avoid that the logic is effected by frame drops
		let time = get_delta_time(ctx); //use time
		self.grass_postion.x += 0.1 * time.as_millis() as f32;
		self.grass_rotation += 0.001 * time.as_millis() as f32;
		Ok(())
	}
}

fn main() -> tetra::Result {
	ContextBuilder::new(format!("{CARGO_PKG_NAME} v{CARGO_PKG_VERSION}"), 640, 480)
		.quit_on_escape(true)
		.multisampling(8) //anti-aliasing
		.stencil_buffer(true)
		.build()
		.expect("error building context")
		.run(GameState::new)
}
