
use core::{fmt::Write};
use embedded_graphics::{
	mono_font::MonoTextStyle,
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
	text::Text
};
use embedded_sprites::{sprite::Sprite};
use heapless::String;
use m3_models::{Card};
use pybadge_high::{
	Color, Display
};

use crate::assets::get_card_image;

pub(crate) mod card_selecter;
pub(crate) mod game_over;
pub(crate) mod driving;

/// count of cards per line
const CARD_LINE_LENGTH: u8 = 6;



#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Activity {
	Selecter,
	Waiting,
	GameOver(m3_models::GameOver)
}

/// draw a card or clearr field is None.
/// The card is drawn at postion i at the line which start the heigh y,
/// with build in line break
fn draw_card(
	i: u8,
	y: u8,
	card: Option<&Card>,
	display: &mut Display,
	text_style_on_card: MonoTextStyle<'_, Color>
) {
	let top_left = Point::new(
		(26 * (i % CARD_LINE_LENGTH) + 2) as i32,
		(y + 38 * (i / CARD_LINE_LENGTH)) as i32
	);
	if let Some(card) = card {
		Sprite::new(top_left, &get_card_image(card))
			.draw(display)
			.unwrap();
		if let Card::Wait(wait_count) = card {
			let mut wait_count_str = String::<3>::new();
			write!(wait_count_str, "{}", wait_count).unwrap();
			Text::new(
				&wait_count_str,
				Point::new(
					(26 * (i % CARD_LINE_LENGTH) + 9) as i32,
					(y + 15 + 38 * (i / CARD_LINE_LENGTH)) as i32
				),
				text_style_on_card
			)
			.draw(display)
			.unwrap();
		}
	} else {
		// clear the postion of the card by filling it with black
		Rectangle::with_corners(top_left, top_left + Point::new(25, 36))
			.into_styled(
				PrimitiveStyleBuilder::new()
					.fill_color(Color::BLACK)
					.build()
			)
			.draw(display)
			.unwrap();
	}
}
