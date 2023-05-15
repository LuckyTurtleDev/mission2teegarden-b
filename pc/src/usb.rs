use anyhow::Context;
use bincode::error::DecodeError;
use m3_models::{MessageToPc, MessageToPyBadge};
use serialport::{available_ports, SerialPort, SerialPortInfo};
use std::{
	iter,
	sync::mpsc::{Receiver, Sender, TryRecvError},
	thread,
	time::Duration
};

struct Player {
	receiver: Receiver<anyhow::Result<MessageToPc>>,
	sender: Sender<MessageToPyBadge>,
	port_name: String
}

#[derive(Default)]
struct Players {
	player1: Option<Player>,
	player2: Option<Player>,
	player3: Option<Player>,
	player4: Option<Player>,
	///uart devices, wich where not classificated as pybadge yets
	possible_players: Vec<Player>
}

impl Players {
	fn iter(&self) -> impl Iterator<Item = &Player> {
		iter::once(&self.player1)
			.chain(iter::once(&self.player2))
			.chain(iter::once(&self.player3))
			.chain(iter::once(&self.player4))
			.flat_map(|f| f.as_ref())
	}

	pub(crate) fn init() -> Players {
		let players = Players::default();
		let mut ports = available_ports().unwrap();
		println!("avaibale ports: {ports:?}");
		ports.retain(|port| {
			!players
				.iter()
				.any(|player| player.port_name == port.port_name)
		});
		let mut possible_players: Vec<Player> = Vec::new();
		for port in ports {
			let (sender_to_pc, receiver_to_pc) = std::sync::mpsc::channel();
			let (sender_to_pybadge, receiver_to_pydage) = std::sync::mpsc::channel();
			let possible_player = Player {
				receiver: receiver_to_pc,
				sender: sender_to_pybadge,
				port_name: port.port_name.clone()
			};
			possible_players.push(possible_player);
			thread::spawn(move || {
				let sender_to_pc: Sender<anyhow::Result<MessageToPc>> = sender_to_pc;
				let receiver_to_pydage: Receiver<MessageToPyBadge> = receiver_to_pydage;
				let pybadge = Pybadge::init(port).unwrap();
				loop {
					match receiver_to_pydage.try_recv() {
						Err(err) => match err {
							TryRecvError::Empty => {},
							TryRecvError::Disconnected => panic!("channel disconnected") /* or should I just break and close the thread? */
						},
						Ok(message) => pybadge.write(&message)
					}
					if let Some(message) = pybadge.try_next_event() {
						if message != MessageToPc::KeepAlive {
							sender_to_pc.send(Ok(message));
						}
					}
				}
			});
		}
		Players {
			player1: None,
			player2: None,
			player3: None,
			player4: None,
			possible_players
		}
	}

	//poll player events
	fn poll(&mut self) {
		todo!()
	}
}

pub(crate) struct Pybadge {
	port: Box<dyn SerialPort>,
	buf: Vec<u8>
}

impl Pybadge {
	fn init(port: SerialPortInfo) -> anyhow::Result<Self> {
		let mut port = serialport::new(port.port_name, 960)
			.timeout(Duration::from_secs(1))
			.open()
			.with_context(|| "Failed to open port")?;
		Ok(Pybadge {
			port,
			buf: Vec::new()
		})
	}

	fn try_next_event(&mut self) -> Option<MessageToPc> {
		match bincode::decode_from_slice(&self.buf, bincode::config::standard()) {
			Ok((event, len)) => {
				self.buf.drain(..len);
				return event;
			},
			Err(err) => {
				match err {
					//we need to wait for more data first
					DecodeError::UnexpectedEnd { .. } => {},
					_ => panic!("Could not decode message\n  {}", err)
				}
			}
		};
		let mut buffer = [0_u8; 16];
		let len = self.port.read(&mut buffer).unwrap();
		if len != 0 {
			//if it even possibel to get len 0? Since it does blocking wait for messages
			let mut new_data: Vec<u8> =
				buffer[..len].iter().map(|f| f.to_owned()).collect();
			println!("recieve data: {new_data:?}");
			self.buf.append(&mut new_data);
		}
		None
	}

	fn write(&mut self, message: &MessageToPyBadge) {
		let data = bincode::encode_to_vec(message, bincode::config::standard()).unwrap();
		println!("send {data:?}");
		self.port.write_all(&data).unwrap();
	}
}
