use m3_models::{MessageToPc, MessageToPyBadge};
use serialport::available_ports;
use std::time::Duration;

pub(crate) fn init() {
	let ports = available_ports().unwrap();
	println!("{ports:?}");
	for port in ports {
		let mut port = serialport::new(port.port_name, 9600)
			.timeout(Duration::from_millis(10))
			.open()
			.expect("Failed to open port");
		let data = bincode::encode_to_vec(
			MessageToPyBadge::ConnectionRequest,
			bincode::config::standard()
		)
		.unwrap();
		port.write_all(&data).unwrap();
		println!("{data:?}");
	}
}
