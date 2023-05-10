use serialport::available_ports;
use std::time::Duration;

pub(crate) fn init() {
	let ports = available_ports().unwrap();
	println!("{ports:?}");
	for port in ports {
		let port = serialport::new(port.port_name, 9600)
			.timeout(Duration::from_millis(10))
			.open()
			.expect("Failed to open port");
	}
}
