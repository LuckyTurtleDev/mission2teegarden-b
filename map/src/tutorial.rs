//! # Mapeditor
//!
//! Misson to Teegarden b allow creating custom maps, by using the [Tiled Map editor](https://www.mapeditor.org/).
//! This does include support for aviable instructions and story.
//!
//! ### Limitaions
//! Theire exist some conditions and limitation how the map is structured:
//! * the map must be finity.
//! * all layers must be finity.
//! * no custom Tileset can be used. So only the Tilesets aviable at github
//! ([`BaseTiles.tsx`](https://github.com/LuckyTurtleDev/m3/blob/main/pc/assets/img/BaseTiles/BaseTiles.tsx),
//! [`ObjectTiles.tsx`](https://github.com/LuckyTurtleDev/m3/blob/main/pc/assets/img/ObjectTiles/ObjectTiles.tsx),
//! [`Player.tsx`](https://github.com/LuckyTurtleDev/m3/blob/main/pc/assets/img/Player/Player.tsx)) can be used.
//! * All layers must be a Tile layer.
//! * The 1. layer must only use Tiles from the `BaseTiles` set.
//! * The 2. layer must only use Tiles from the `ObjectTiles` set.
//! * The 3. layer must only use Tiles from the `Player` set.
//! * If a field at layer 1. is not set `Grass` is used as default.
//! * If player `i` have a start postion. All player `<i` must also have a start postion.
//! * At least player `i` must have a start postion.
//! * If global goal was not set, each player (which have a start postion), must have a player goal.
//!
//! ### Aviable Instructions
//! Aviable instruction can be added, by adding a "Custom properties" with type `int` to the Map.
//! The propertie must be named like the fields of the [`AvailableCards`](crate::AvailableCards) struct.
//! If no propertie for an instruction `0` is used as default.
//! Keep in mind that the player can only use `12` cards in total.
//!
//! ### Story
//! An optional story can be added by creating a map propertie called `story` from type `string`
//! For decoding the story inside the propertie the [toml](https://toml.io/) format is used.
//! Currently only story elments before and after the level are supported.
//!
//! Take a look at this example story:
//! ```
//! # let toml = r#"
//! [[pre_level]]
//! text = "hi, I am the captain ..."
//! profil = "Captain"
//! background = "OuterSpace"
//!
//! [[pre_level]]
//! text = "now it is you turn!"
//!
//! [[after_level]]
//! text = "You have mastered the challenge!"
//! profil = "Captain"
//! # "#;
//! # let _config: m3_map::story::Story = basic_toml::from_str(&toml).unwrap_or_else(|err| panic!("{}", err));
//! ```
//! The story has two lists `pre_level` and `after_level`, both are optional.
//! Each list inculde zero or more [`Speech`s](crate::story::Speech).
//! The [`Speech`s](crate::story::Speech) from `pre_level` are shown before the level starts,
//! the ones from `after_level` are show, after the level was finish successfully.
//! A [`Speech`](crate::story::Speech) exist out of a `text`, a `profil`picture and a `background`.
//! The last two are optional.
//! `profil` defined the picture, which is show left from the text.
//! All variants of [`Character`](`crate::story::Character`) can be used for this.
//! If `profil` is not set, no picture will be shown.
//! `background` define the background with is show above the text.
//! All variants of [`Background`](`crate::story::Background`) can be used for this.
//! If `background` is not set, the level will be shown.
//!
//! For more info see the [`Story`](`crate::story::Story`) struct.