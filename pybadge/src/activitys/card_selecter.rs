use crate::State;

use embedded_graphics::prelude::*;
use embedded_sprites::{image::Image, include_image, sprite::Sprite};

use konst::result::unwrap_ctx;
use m3_models::Card;
use pybadge_high::{
	buttons::{Button, Event},
	Color
};
use strum::IntoEnumIterator;

#[include_image]
const IMG_CARD_LEFT: Image<Color> = "pybadge/img/Left.png";
#[include_image]
const IMG_CARD_RIGHT: Image<Color> = "pybadge/img/Right.png";
#[include_image]
const IMG_CARD_MOVE: Image<Color> = "pybadge/img/Move.png";
#[include_image]
const IMG_CARD_STOP: Image<Color> = "pybadge/img/Stop.png";
#[include_image]
const IMG_CARD_WAIT: Image<Color> = "pybadge/img/Wait.png";
#[include_image]
const IMG_CARD_SELETED: Image<Color> = "pybadge/img/CardSelected.png";
#[include_image]
const IMG_CARD_FRAME: Image<Color> = "pybadge/img/CardFrame.png";
const IMG_EMPTY: Image<Color> =
	unwrap_ctx!(Image::new(&[Color::new(0, 0, 0)], &[1, 1], 1, 1));

fn get_card_image(card: Card) -> Image<'static, Color> {
	match card {
		Card::Left => IMG_CARD_LEFT,
		Card::Right => IMG_CARD_RIGHT,
		Card::MotorOn => IMG_CARD_MOVE,
		Card::MotorOff => IMG_CARD_STOP,
		Card::Wait(_) => IMG_CARD_WAIT
	}
}

pub(crate) fn init(state: &mut State) {
	state.display.clear(Color::BLACK).unwrap();
	for (i, card) in Card::iter().enumerate() {
		Sprite::new(Point::new((26 * i + 2) as i32, 91), &get_card_image(card))
			.draw(&mut state.display)
			.unwrap();
	}
	state.cursor = (0, 1);
}

pub(crate) fn update(state: &mut State) {
	if state.buttons.some_pressed() {
		let last_cursor_pos = state.cursor;
		for event in state.buttons.events() {
			if let Event::Pressed(button) = event {
				match button {
					Button::Right => state.cursor.0 += 1,
					Button::Left => state.cursor.0 -= 1,
					_ => {}
				}
			}
		}
		if state.cursor.0 > 100 {
			state.cursor.0 = 4 //TODO: do not hardcode this
		}
		if state.cursor.0 > 4 {
			//TODO: do not hardcode this
			state.cursor.0 = 0
		}
		Sprite::new(
			Point::new((26 * last_cursor_pos.0 + 2) as i32, 91),
			&IMG_CARD_FRAME
		)
		.draw(&mut state.display)
		.unwrap();
		Sprite::new(
			Point::new((26 * state.cursor.0 + 2) as i32, 91),
			&IMG_CARD_SELETED
		)
		.draw(&mut state.display)
		.unwrap();
	}
}
