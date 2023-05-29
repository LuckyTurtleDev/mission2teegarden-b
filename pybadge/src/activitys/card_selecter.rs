use crate::State;
use core::fmt::Write;
use embedded_graphics::{prelude::*, text::Text};
use embedded_sprites::{image::Image, include_image, sprite::Sprite};
use heapless::{String, Vec};
use konst::result::unwrap_ctx;
use m3_models::{AvailableCards, Card};
use pybadge_high::{
	buttons::{Button, Event},
	Color
};
use strum::{EnumCount, IntoEnumIterator};

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
	let mut count_str = String::<3>::new();
	//draw only cards, which are aviable for this level
	for (i, (count, card)) in Card::iter()
		.filter_map(|card| {
			if state.init_avaiable_cards.card_count(&card) == 0 {
				None
			} else {
				Some((state.avaiable_cards.card_count(&card), card))
			}
		})
		.enumerate()
	{
		Sprite::new(Point::new((26 * i + 2) as i32, 91), &get_card_image(card))
			.draw(&mut state.display)
			.unwrap();
		if let Card::Wait(_) = card {
			Text::new(
				"i",
				Point::new((26 * i + 9) as i32, 106),
				state.text_style_large_black
			)
			.draw(&mut state.display)
			.unwrap();
		}
		count_str.clear();
		write!(count_str, "{count}");
		Text::new(
			&count_str,
			Point::new((26 * i + 9) as i32, 86),
			state.text_style_large
		)
		.draw(&mut state.display)
		.unwrap();
	}
	state.cursor = (0, 1);
}

pub(crate) fn update(state: &mut State) {
	if state.buttons.some_pressed() {
		let last_cursor_pos = state.cursor;
		let mut add_card = false;
		for event in state.buttons.events() {
			if let Event::Pressed(button) = event {
				match button {
					/// cursor pos was eventuell changed and is now invalid
					/// we need to make it valid again first
					Button::A => add_card = true,
					Button::B => {
						state.solution.pop();
					},
					Button::Right => state.cursor.0 += 1,
					Button::Left => state.cursor.0 -= 1,
					_ => {}
				}
			}
		}
		if state.cursor.0 == u8::MAX {
			state.cursor.0 = state.card_type_count - 1;
		}
		if state.cursor.0 >= state.card_type_count {
			state.cursor.0 = 0;
		}
		if add_card {
			//update card state
			for (i, card) in Card::iter()
				.filter(|card| state.init_avaiable_cards.card_count(&card) != 0)
				.enumerate()
			{
				// the card below the cursor
				if i as u8 == state.cursor.0 {
					let count: &mut u8 = state.avaiable_cards.card_count_mut(&card);
					if count == &0_u8 {
						continue;
					}
					*count -= 1;
					state.solution.push();
				}
			}
			//draw new card
			if let Some(card) = state.solution.last()
			//solution can be added
			{
				let len = state.solution.len();
				Sprite::new(Point::new((26 * len + 2) as i32, 2), &get_card_image(card))
					.draw(&mut state.display)
					.unwrap();
			}
		}
		if last_cursor_pos != state.cursor {
			let number = String::<2>::new();
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
}
