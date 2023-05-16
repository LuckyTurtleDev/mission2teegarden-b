#![no_std]
#![no_main]

use bincode::{decode_from_slice, encode_into_slice, error::DecodeError};
use embedded_graphics::{draw_target::DrawTarget, prelude::*};
use heapless::Vec;
use m3_models::{MessageToPc, MessageToPyBadge, ToPcProtocol, ToPybadgeProtocol};
use pybadge_high::{prelude::*, Color, PyBadge};

mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

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

#[entry]
fn main() -> ! {
	let mut usb_data = Vec::<u8, 128>::new();
	let mut pybadge = PyBadge::take().unwrap();
	let mut delay = pybadge.delay;
	let mut display = pybadge.display;
	display.clear(Color::BLACK).unwrap();
	usb::init(pybadge.usb_builder);
	//wait for connection with pc;
	while read_events(&mut usb_data)
		.iter()
		.any(|f| f == &MessageToPyBadge::Protocol(ToPybadgeProtocol::ConnectionRequest))
	{
		delay.delay_ms(50_u16);
		usb::read(&mut usb_data);
	}
	send_event(MessageToPc::Protocol(ToPcProtocol::ConnectionResponse));
	//Todo: do not throw away event, wihich are directly send after ConnectionRequest
	loop {
		send_event(MessageToPc::KeepAlive);
		usb::read(&mut usb_data);
	}
}
