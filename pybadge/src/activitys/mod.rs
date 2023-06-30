use crate::assets::{
	get_card_image, CARD_HEIGHT, CARD_WIHDT, IMG_CARD_FRAME, IMG_CARD_SELETED
};
use core::fmt::Write;
use embedded_graphics::{
	mono_font::MonoTextStyle,
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
	text::Text
};
use embedded_sprites::sprite::Sprite;
use heapless::String;
use mission2teegarden_b_models::Card;
use pybadge_high::{Color, Display};

pub(crate) mod card_selecter;
pub(crate) mod driving;
pub(crate) mod game_over;

/// count of cards per line
const CARD_LINE_LENGTH: u8 = 6;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Activity {
	Selecter,
	Driving,
	Waiting,
	GameOver(mission2teegarden_b_models::GameOver)
}

enum DrawObject<'a> {
	/// Clear card, by over drawing it with with black
	Clear,
	Card(&'a Card),
	/// highlight a card by coloring the frame of the card.
	Cursor,
	/// redraw the frame, of a card.
	/// Can be used to clear `Cursor` selection.
	Frame
}

/// draw a card or clearr field is None.
/// The card is drawn at postion i at the line which start the heigh y,
/// with build in line break
fn draw_card(
	i: u8,
	y: u8,
	draw_object: DrawObject<'_>,
	display: &mut Display,
	text_style_on_card: MonoTextStyle<'_, Color>
) {
	let top_left = Point::new(
		(CARD_WIHDT * (i % CARD_LINE_LENGTH) + 2) as i32,
		(y + CARD_HEIGHT * (i / CARD_LINE_LENGTH)) as i32
	);
	match draw_object {
		DrawObject::Card(card) => {
			Sprite::new(top_left, &get_card_image(card))
				.draw(display)
				.unwrap();
			if let Card::Wait(wait_count) = card {
				let mut wait_count_str = String::<3>::new();
				write!(wait_count_str, "{}", wait_count).unwrap();
				Text::new(
					&wait_count_str,
					Point::new(
						(CARD_WIHDT * (i % CARD_LINE_LENGTH) + 9) as i32,
						(y + 15 + CARD_HEIGHT * (i / CARD_LINE_LENGTH)) as i32
					),
					text_style_on_card
				)
				.draw(display)
				.unwrap();
			}
		},
		DrawObject::Clear => {
			// clear the postion of the card by filling it with black
			Rectangle::with_corners(
				top_left,
				top_left + Point::new(CARD_WIHDT as i32, CARD_HEIGHT as i32)
			)
			.into_styled(
				PrimitiveStyleBuilder::new()
					.fill_color(Color::BLACK)
					.build()
			)
			.draw(display)
			.unwrap();
		},
		DrawObject::Cursor => {
			Sprite::new(top_left, &IMG_CARD_SELETED)
				.draw(display)
				.unwrap();
		},
		DrawObject::Frame => Sprite::new(top_left, &IMG_CARD_FRAME)
			.draw(display)
			.unwrap()
	}
}
