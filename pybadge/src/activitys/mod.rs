use crate::State;
use embedded_graphics::prelude::*;
use embedded_sprites::{image::Image, include_image, sprite, sprite::Sprite};
use m3_models::Card;

pub(crate) mod card_selecter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Activity {
	Selecter,
	Waiting
}
