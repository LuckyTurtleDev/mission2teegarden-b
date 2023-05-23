#![no_std]
#![no_main]

use activitys::Activity;
use bincode::{decode_from_slice, encode_into_slice, error::DecodeError};
use embedded_graphics::{
	draw_target::DrawTarget,
	mono_font::{ascii::FONT_6X10, MonoTextStyle},
	prelude::*,
	text::Text
};

use heapless::Vec;

use m3_models::{
	AvailableCards, Key, MessageToPc, MessageToPyBadge, ToPcGameEvent,
	ToPcProtocol, ToPybadgeProtocol
};
use pybadge_high::{prelude::*, Buttons, Color, Display, PyBadge};
use strum::IntoEnumIterator;

mod activitys;
mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

const DISPLAY_WIDHT: u16 = 160;
const DISPLAY_HIGHT: u16 = 128;

fn read_events(usb_data: &mut Vec<u8, 128>) -> Vec<MessageToPyBadge, 10> {
	let mut events = Vec::<MessageToPyBadge, 10>::new();
	//vec has max size of 10
	for _ in 0..10 {
		match decode_from_slice(usb_data.as_slice(), bincode::config::standard()) {
			Ok((event, len)) => {
				events.push(event).unwrap();
				//remove readed bytes
				let remaining_data: Vec<u8, 128> =
					usb_data[..len].iter().copied().collect();
				usb_data.clear();
				usb_data
					.extend_from_slice(remaining_data.as_slice())
					.unwrap();
			},
			Err(err) => {
				match err {
					//we need to wait for more data first
					DecodeError::UnexpectedEnd { .. } => break,
					_ => panic!("Could not decode message\n  {}", err)
				}
			}
		};
	}
	events
}

fn send_event(event: MessageToPc) {
	let mut buf = [0_u8; 128];
	let len = encode_into_slice(event, &mut buf, bincode::config::standard()).unwrap();
	usb::wirte(&buf[..len]);
}

fn send_button_state(buttons: &Buttons) {
	if buttons.a_pressed() {
		send_event(MessageToPc::GameEvent(ToPcGameEvent::KeyPressed(Key::A)));
	}
	if buttons.b_pressed() {
		send_event(MessageToPc::GameEvent(ToPcGameEvent::KeyPressed(Key::B)));
	}
	if buttons.up_pressed() {
		send_event(MessageToPc::GameEvent(ToPcGameEvent::KeyPressed(Key::Up)));
	}
	if buttons.down_pressed() {
		send_event(MessageToPc::GameEvent(ToPcGameEvent::KeyPressed(Key::Down)));
	}
	if buttons.left_pressed() {
		send_event(MessageToPc::GameEvent(ToPcGameEvent::KeyPressed(Key::Left)));
	}
	if buttons.right_pressed() {
		send_event(MessageToPc::GameEvent(ToPcGameEvent::KeyPressed(
			Key::Right
		)));
	}
}

struct State {
	display: Display,
	buttons: Buttons,
	avaiable_cards: AvailableCards,
	activity: Activity
}

impl State {
	fn init_activity(&mut self) {
		match self.activity {
			Activity::Waiting => {},
			Activity::Selecter => activitys::card_selecter::init(self)
		}
	}
	fn update_draw(&mut self) {}
}

#[entry]
fn main() -> ! {
	let text_style = MonoTextStyle::new(&FONT_6X10, Color::WHITE);
	let mut usb_data = Vec::<u8, 128>::new();
	let pybadge = PyBadge::take().unwrap();
	let mut delay = pybadge.delay;
	let mut display = pybadge.display;
	let mut buttons = pybadge.buttons;
	display.clear(Color::BLACK).unwrap();
	Text::new(
		"Please connect pybadge to \n  pc then start the game",
		Point::new(5, 50),
		text_style
	)
	.draw(&mut display)
	.ok();
	usb::init(pybadge.usb_builder);
	//wait for connection with pc;
	while !read_events(&mut usb_data)
		.iter()
		.any(|f| f == &MessageToPyBadge::Protocol(ToPybadgeProtocol::ConnectionRequest))
	{
		delay.delay_ms(50_u16);
		usb::read(&mut usb_data);
	}
	display.clear(Color::BLACK).unwrap();
	Text::new(
		"Press anykey to \n join the game",
		Point::new(35, 50),
		text_style
	)
	.draw(&mut display)
	.ok();
	while !buttons.some_pressed() {
		buttons.update();
	}
	send_event(MessageToPc::Protocol(ToPcProtocol::ConnectionResponse));
	display.clear(Color::BLACK).unwrap();
	Text::new("look at the pc screen", Point::new(15, 50), text_style)
		.draw(&mut display)
		.ok();
	//Todo: do not throw away event, wihich are directly send after ConnectionRequest
	let avaiable_cards = AvailableCards {
		left: 3,
		right: 2,
		..Default::default()
	};
	let mut state = State {
		display,
		buttons,
		avaiable_cards,
		activity: Activity::Selecter
	};
	let mut last_activity = Activity::Waiting;
	loop {
		send_event(MessageToPc::KeepAlive);
		usb::read(&mut usb_data);
		state.buttons.update();
		send_button_state(&state.buttons);
		if last_activity != state.activity {
			state.init_activity();
			last_activity = state.activity;
		}
		state.update_draw();
		delay.delay_ms(1000_u16);
	}
}
