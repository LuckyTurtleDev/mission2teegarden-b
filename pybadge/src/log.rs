use crate::send_event;
use core::{fmt::Write, write};
use heapless::String;
use mission2teegarden_b_models::{Log, ToPcProtocol};

// This is extremly ugl
// and should to refactor
/// Sends an log level to pc (max 100 bytes)
#[allow(dead_code)]
pub(crate) fn debug<S>(message: S)
where
	S: core::fmt::Display
{
	let mut message_string = String::<100>::new();
	write!(message_string, "{message}").ok();
	let length = message_string.len() as u16;
	let mut message_bytes = [0_u8; 100];
	for (i, byte) in message_string.into_bytes().into_iter().enumerate() {
		message_bytes[i] = byte;
	}
	send_event(mission2teegarden_b_models::MessageToPc::Protocol(
		ToPcProtocol::Log(Log {
			length,
			message: message_bytes
		})
	));
}
