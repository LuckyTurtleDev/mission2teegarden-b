use crate::{
	activitys::draw_card,
	assets::{IMG_CARD_FRAME, IMG_CARD_SELETED},
	send_event, State
};
use core::{fmt::Write, mem};
use embedded_graphics::{
	mono_font::MonoTextStyle,
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
	text::Text
};
use embedded_sprites::{image::Image, include_image, sprite::Sprite};
use heapless::String;
use m3_models::{Card, MessageToPc, ToPcGameEvent};
use pybadge_high::{
	buttons::{Button, Event},
	Color, Display
};
use strum::IntoEnumIterator;

pub(crate) fn init(state: &mut State<'_>) {
    state.display.clear(Color::BLACK).unwrap();
	for (i, card) in state.submitted_solution.iter().enumerate() {
		draw_card(
			i as u8,
			1,
			Some(card),
			&mut state.display,
			state.text_style_on_card
		);
	}
}

pub(crate) fn update(state: &mut State<'_>) {}
