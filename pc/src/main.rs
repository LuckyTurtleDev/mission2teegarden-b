use tetra::{graphics, graphics::Color, Context, ContextBuilder, State};
type Vec2 = vek::vec::repr_c::vec2::Vec2<f32>;
use m3_macro::include_map;
use m3_map::Map;
use once_cell::sync::Lazy;
use tetra::{
	graphics::{DrawParams, Texture},
	time::get_delta_time
};

mod tiles;
use tiles::{MapBaseTile, Textures};

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
			level: Some(*(LEVELS.first()).unwrap())
		})
	}
}

impl State for GameState {
	//draw the current state
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

		match self.level {
			None => todo!(),
			Some(map) => {
				for (x,y, tile) in map.iter_base_layer(){
					let texture = tile.texture(&self.textures);
					
				}
			}
		}

		//moving sprite
		MapBaseTile::Grass
			.texture(&self.textures)
			.draw(ctx, self.grass_postion);

		//rotating sprite
		MapBaseTile::Grass.texture(&self.textures).draw(
			ctx,
			DrawParams::new()
				.origin(Vec2::new(
					(MapBaseTile::Grass.texture(&self.textures).width() / 2) as f32,
					(MapBaseTile::Grass.texture(&self.textures).height() / 2) as f32
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
	println!("{:?}", LEVELS[0]);
	ContextBuilder::new(format!("{CARGO_PKG_NAME} v{CARGO_PKG_VERSION}"), 640, 480)
		.quit_on_escape(true)
		.multisampling(8) //anti-aliasing
		.stencil_buffer(true)
		.build()
		.expect("error building context")
		.run(GameState::new)
}
