use embedded_sprites::{image::Image, include_image};

use mission2teegarden_b_models::Card;
use pybadge_high::Color;

pub(crate) const CARD_WIHDT: u8 = 25;
pub(crate) const CARD_HEIGHT: u8 = 36;

#[include_image]
pub(crate) const IMG_CARD_LEFT: Image<'_, Color> = "pybadge/img/Left.png";
#[include_image]
pub(crate) const IMG_CARD_RIGHT: Image<'_, Color> = "pybadge/img/Right.png";
#[include_image]
pub(crate) const IMG_CARD_MOVE: Image<'_, Color> = "pybadge/img/Move.png";
#[include_image]
pub(crate) const IMG_CARD_STOP: Image<'_, Color> = "pybadge/img/Stop.png";
#[include_image]
pub(crate) const IMG_CARD_WAIT: Image<'_, Color> = "pybadge/img/Wait.png";
#[include_image]
pub(crate) const IMG_CARD_SELETED: Image<'_, Color> = "pybadge/img/CardSelected.png";
#[include_image]
pub(crate) const IMG_CARD_FRAME: Image<'_, Color> = "pybadge/img/CardFrame.png";

pub(crate) fn get_card_image(card: &Card) -> Image<'static, Color> {
	match card {
		Card::Left => IMG_CARD_LEFT,
		Card::Right => IMG_CARD_RIGHT,
		Card::MotorOn => IMG_CARD_MOVE,
		Card::MotorOff => IMG_CARD_STOP,
		Card::Wait(_) => IMG_CARD_WAIT
	}
}
