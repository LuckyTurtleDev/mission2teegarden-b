use crate::{CARGO_PKG_NAME, CARGO_PKG_VERSION};
use heapless::{Deque, Vec};
use pybadge_high::{
	cortex_m::interrupt,
	usb::{Usb, UsbBuilder}
};

static mut USB_DATA_IN: Vec<u8, 128> = Vec::new();
static mut USB_DEV: Option<Usb> = None;

fn interupt() {
	let mut read_data = [0_u8; 64];
	let usb_dev = unsafe { USB_DEV.as_mut().unwrap() };
	if let Ok(len) = usb_dev.read(&mut read_data) {
		unsafe {
			USB_DATA_IN.extend_from_slice(&read_data[..len]);
		}
	}
}

pub(crate) fn init(builder: UsbBuilder) {
	let mut usb = builder
		.product(stringify!("{CARGO_PKG_NAME}-{CARGO_CARGO_PKG_VERSION}"))
		.build();
	usb.set_interrupt(interupt);
	unsafe {
		USB_DEV = Some(usb);
		USB_DEV.as_mut().unwrap().enable_interrupt()
	};
}

pub(crate) fn read(data: &mut Vec<u8, 128>) {
	interrupt::free(|_| unsafe {
		data.clone_from_slice(&USB_DATA_IN);
		USB_DATA_IN.clear();
	});
}

pub(crate) fn wirte(data: &[u8]) {
	interrupt::free(|_| unsafe {
		USB_DEV.as_mut().unwrap().write(data).unwrap();
	});
}
