use m3_macro::include_map;
use macroquad::{prelude::ImageFormat, texture::Texture2D};
use once_cell::sync::Lazy;

macro_rules! include_music {
	($file:expr) => {
		Song {
			file_name: $file,
			data: include_bytes!(concat!("../assets/sound/music/", $file))
		}
	};
}

pub(crate) const MUSIC: Music = Music {
	/// used as title music
	titel_music: include_music!("HoliznaCC0 - Mutant Club.mp3"),
	background_music: &[
		include_music!("HoliznaCC0 - Ancient Memories.mp3"),
		include_music!("HoliznaCC0 - Dance Of The Dead.mp3"),
		include_music!("HoliznaCC0 - Dusty Attic.mp3"),
		include_music!("HoliznaCC0 - Earth.mp3"),
		include_music!("HoliznaCC0 - Little Green Men.mp3"),
		include_music!("HoliznaCC0 - Mercury.mp3"),
		include_music!("HoliznaCC0 - Saturn.mp3"),
		include_music!("HoliznaCC0 - Sky Fish.mp3"),
		include_music!("HoliznaCC0 - Somethings Out There.mp3"),
		include_music!("HoliznaCC0 - Track 1.mp3")
	]
};

pub(crate) const SOUNDS: Sounds = Sounds {};

pub(crate) struct Song {
	pub(crate) file_name: &'static str,
	pub(crate) data: &'static [u8]
}

pub(crate) struct Music {
	pub(crate) titel_music: Song,
	pub(crate) background_music: &'static [Song]
}

pub(crate) struct Sounds {}

///store maps as String binary format
///call `Map::from_str()`
pub(crate) static LEVELS: Lazy<Vec<&str>> = Lazy::new(|| {
	vec![
		include_map!("pc/assets/level/001.tmx"),
		include_map!("pc/assets/level/002.tmx"),
		include_map!("pc/assets/level/003.tmx"),
		include_map!("pc/assets/level/004.tmx"),
	]
});

#[allow(clippy::redundant_closure)] //false positive?
pub(crate) static TEXTURES: Lazy<Textures> = Lazy::new(|| Textures::init());

pub(crate) trait GetTexture {
	fn texture(&self) -> Texture2D;
}

///Store all Textures
pub(crate) struct Textures {
	pub(crate) grass: Texture2D,
	pub(crate) grass_corner_sand: Texture2D,
	pub(crate) half_grass_sand: Texture2D,
	pub(crate) sand: Texture2D,
	pub(crate) sand_corner_grass: Texture2D,
	pub(crate) stone: Texture2D,
	pub(crate) player1_car: Texture2D,
	pub(crate) player2_car: Texture2D,
	pub(crate) player3_car: Texture2D,
	pub(crate) player4_car: Texture2D,
	pub(crate) global_goal: Texture2D,
	pub(crate) player1_goal: Texture2D,
	pub(crate) player2_goal: Texture2D,
	pub(crate) player3_goal: Texture2D,
	pub(crate) player4_goal: Texture2D,
	pub(crate) captain: Texture2D,
	pub(crate) outer_space: Texture2D
}

impl Textures {
	///init all Textures
	pub(crate) fn init() -> Textures {
		Textures {
			grass: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/grass.png"),
				Some(ImageFormat::Png)
			),
			grass_corner_sand: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/grass_corner_sand.png"),
				Some(ImageFormat::Png)
			),
			half_grass_sand: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/half_grass_sand.png"),
				Some(ImageFormat::Png)
			),
			sand: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/sand.png"),
				Some(ImageFormat::Png)
			),
			sand_corner_grass: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/sand_corner_grass.png"),
				Some(ImageFormat::Png)
			),
			stone: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/ObjectTiles/stone.png"),
				Some(ImageFormat::Png)
			),
			player1_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player1_car.png"),
				Some(ImageFormat::Png)
			),
			player2_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player2_car.png"),
				Some(ImageFormat::Png)
			),
			player3_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player3_car.png"),
				Some(ImageFormat::Png)
			),
			player4_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player4_car.png"),
				Some(ImageFormat::Png)
			),
			global_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/goal.png"),
				Some(ImageFormat::Png)
			),
			player1_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player1_goal.png"),
				Some(ImageFormat::Png)
			),
			player2_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player2_goal.png"),
				Some(ImageFormat::Png)
			),
			player3_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player3_goal.png"),
				Some(ImageFormat::Png)
			),
			player4_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player4_goal.png"),
				Some(ImageFormat::Png)
			),
			captain: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Character/captain.png"),
				Some(ImageFormat::Png)
			),
			outer_space: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Backgrounds/outer_space.png"),
				Some(ImageFormat::Png)
			)
		}
	}

	pub(crate) fn get_player_textures(&self) -> Vec<Texture2D> {
		vec![
			self.player1_car,
			self.player2_car,
			self.player3_car,
			self.player4_car,
		]
	}
}
