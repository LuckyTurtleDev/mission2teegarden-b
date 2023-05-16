use tetra::{
	graphics::{self, Color},
	window::get_size,
	Context, ContextBuilder, State
};
type Vec2 = vek::vec::repr_c::vec2::Vec2<f32>;
use log::info;
use m3_macro::include_map;
use m3_map::Map;
use once_cell::sync::Lazy;
use tetra::{
	graphics::{DrawParams, Texture},
	time::get_delta_time
};

mod tiles;
use tiles::{MapBaseTile, Textures};

mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

static LEVELS: Lazy<Vec<Map>> =
	Lazy::new(|| vec![include_map!("pc/assets/level/001.tmx")]);

trait GetTexture<'a> {
	fn texture(&self, textures: &'a Textures) -> &'a Texture;
}

//https://tetra.seventeencups.net/tutorial

struct GameState {
	textures: Textures,
	grass_postion: Vec2,
	grass_rotation: f32,
	level: Option<Map>
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let textures = Textures::init(ctx);
		Ok(GameState {
			textures,
			grass_postion: Vec2::default(),
			grass_rotation: 0.0,
			level: Some(LEVELS.first().unwrap().to_owned())
		})
	}
}

impl State for GameState {
	//draw the current state
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
		let window_size = get_size(ctx);

		match &self.level {
			None => todo!(),
			Some(map) => {
				let ratio = Vec2::new(
					(window_size.0 / map.width as i32) as f32,
					(window_size.1 / map.height as i32) as f32
				);
				for (x, y, tile) in map.iter_all() {
					let texture = tile.texture(&self.textures);
					texture.draw(
						ctx,
						DrawParams::new()
							.scale(Vec2::new(
								ratio.x / texture.width() as f32,
								ratio.y / texture.height() as f32
							))
							.position(Vec2::new(x as f32 * ratio.x, y as f32 * ratio.y))
					);
				}
			}
		}

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
	let players = usb::Players::init();
	my_env_logger_style::just_log();
	info!("{:?}", LEVELS[0]);
	ContextBuilder::new(format!("{CARGO_PKG_NAME} v{CARGO_PKG_VERSION}"), 1280, 720)
		.quit_on_escape(true)
		.multisampling(8) //anti-aliasing
		.stencil_buffer(true)
		.build()
		.expect("error building context")
		.run(GameState::new)
}
