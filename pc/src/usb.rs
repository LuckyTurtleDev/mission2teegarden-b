use anyhow::Context;
use bincode::error::DecodeError;
use m3_models::{MessageToPc, MessageToPyBadge, ToPcGameEvent};
use serialport::{available_ports, SerialPort, SerialPortInfo, ClearBuffer};
use std::{
	iter,
	sync::mpsc::{Receiver, Sender, TryRecvError},
	thread,
	time::Duration
};

struct Player {
	receiver: Receiver<MessageToPc>,
	sender: Sender<MessageToPyBadge>,
	port_name: String
}

#[derive(Default)]
pub(crate) struct Players {
	players: [Option<Player>;4],
	///uart devices, wich where not classificated as pybadge yets
	possible_players: Vec<Player>
}

impl Players {

	pub(crate) fn init() -> Players {
		let players = Players::default();
		let mut ports = available_ports().unwrap();
		println!("avaibale ports: {ports:?}");
		ports.retain(|port| {
			!players.players.iter().flatten()
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
				let sender_to_pc: Sender<MessageToPc> = sender_to_pc;
				let receiver_to_pydage: Receiver<MessageToPyBadge> = receiver_to_pydage;
				let mut  pybadge = Pybadge::init(port).unwrap();
				//clean connection
				pybadge.port.clear(ClearBuffer::All);
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
							sender_to_pc.send(message);
						}
					}
				}
			});
		}
		Players {
			players: [None,None,None,None],
			possible_players
		}
	}

	///get aviable player events.
	///Element i of return value repsent player i.
	///ELement is None if no pybade is connected for player i.
	fn get_events(&mut self) -> [Option<Vec<ToPcGameEvent>>;4] {
		if self.players.iter().any(|f| f.is_none()) {
			!todo!()
		}
		let mut events = [None, None, None, None];
		for (i, player) in self.players.iter().enumerate(){
			if let Some(player) = player {
				let mut events_of_player = Vec::new();
				match player.receiver.try_recv(){
					Ok(event) => match event{
						MessageToPc::GameEvent(event) => events_of_player.push(event),
						MessageToPc::Protocol(protocol) => todo!(),
						MessageToPc::KeepAlive => {},
					},
					Err(err) => todo!(),
				}
				events[i]= Some(events_of_player);
			}
		}
		events
	}
}

pub(crate) struct Pybadge {
	port: Box<dyn SerialPort>,
	buf: Vec<u8>
}

impl Pybadge {
	fn init(port: SerialPortInfo) -> anyhow::Result<Self> {
		let port = serialport::new(port.port_name, 960)
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
