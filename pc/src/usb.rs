use anyhow::Context;
use bincode::error::DecodeError;
use log::{debug, info, trace};
use m3_models::{
	MessageToPc, MessageToPyBadge, ToPcGameEvent, ToPybadgeProtocol, ToPypadeGameEvent
};
use serialport::{available_ports, ClearBuffer, SerialPort, SerialPortInfo};
use std::{
	io,
	sync::mpsc::{Receiver, Sender, TryRecvError},
	thread,
	time::Duration
};

#[derive(Debug)]
pub(crate) struct Player {
	receiver: Receiver<MessageToPc>,
	sender: Sender<MessageToPyBadge>,
	port_name: String
}

impl Player {
	/// send a game event to the player
	pub(crate) fn send_events(&self, game_event: ToPypadeGameEvent) {
		self.sender
			.send(MessageToPyBadge::GameEvent(game_event))
			.expect("pybdage communication was closed");
	}
}

#[derive(Default, Debug)]
pub(crate) struct Players {
	/// found players. Might can become `Some` later,
	/// if a new player does join.
	///
	/// The connection establishment is already finish
	/// and message can be send/receive to/from player,
	/// if entry is `Some`.
	pub(crate) players: [Option<Player>; 4],
	///uart devices, wich where not classificated as pybadge yet
	possible_players: Vec<Player>
}

impl Players {
	pub(crate) fn init() -> Players {
		let players = Players::default();
		let mut ports = available_ports().unwrap();
		debug!("avaibale ports: {ports:?}");
		ports.retain(|port| {
			!players
				.players
				.iter()
				.flatten()
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
			thread::Builder::new()
				.name(port.port_name.clone())
				.spawn(move || {
					let sender_to_pc: Sender<MessageToPc> = sender_to_pc;
					let receiver_to_pydage: Receiver<MessageToPyBadge> =
						receiver_to_pydage;
					let mut pybadge = Pybadge::init(port).unwrap();
					//clean connection
					pybadge.port.clear(ClearBuffer::All).unwrap();
					pybadge.write(&MessageToPyBadge::Protocol(
						ToPybadgeProtocol::ConnectionRequest
					));
					loop {
						match receiver_to_pydage.try_recv() {
							Err(err) => match err {
								TryRecvError::Empty => {},
								TryRecvError::Disconnected => {
									panic!("channel disconnected")
								} /* or should I just break and close the thread? */
							},
							Ok(message) => pybadge.write(&message)
						}
						if let Some(message) = pybadge.try_next_event() {
							if message != MessageToPc::KeepAlive {
								sender_to_pc.send(message).unwrap();
							}
						}
					}
				})
				.unwrap();
		}
		Players {
			players: [None, None, None, None],
			possible_players
		}
	}

	///get aviable player events.
	///Element i of return value repsent player i.
	///ELement is None if no pybade is connected for player i.
	pub(crate) fn get_events(&mut self) -> [Option<Vec<ToPcGameEvent>>; 4] {
		if self.players.iter().any(|f| f.is_none()) {
			//check if some of the serial ports a pybadge and it as player
			let mut pos = None;
			for (i, possible_player) in self.possible_players.iter().enumerate() {
				let message = match possible_player.receiver.try_recv() {
					Ok(value) => value,
					Err(err) => match err {
						TryRecvError::Empty => continue,
						TryRecvError::Disconnected => panic!("channel disconnected")
					}
				};
				if MessageToPc::Protocol(m3_models::ToPcProtocol::ConnectionResponse)
					== message
				{
					pos = Some(i);
					break;
				}
			}
			if let Some(pos) = pos {
				let new_player = self.possible_players.remove(pos);
				info!("player join from port {}", new_player.port_name);
				*self.players.iter_mut().find(|f| f.is_none()).unwrap() =
					Some(new_player);
			}
		}
		let mut events = [None, None, None, None];
		for (i, player) in self.players.iter().enumerate() {
			if let Some(player) = player {
				let mut events_of_player = Vec::new();
				match player.receiver.try_recv() {
					Ok(event) => match event {
						MessageToPc::GameEvent(event) => events_of_player.push(event),
						MessageToPc::Protocol(_protocol) => todo!(),
						MessageToPc::KeepAlive => {}
					},
					Err(err) => match err {
						TryRecvError::Empty => continue,
						TryRecvError::Disconnected => panic!("channel disconnected")
					}
				}
				events[i] = Some(events_of_player);
			}
		}
		events
	}
}

pub(crate) struct Pybadge {
	port: Box<dyn SerialPort>,
	port_name: String,
	buf: Vec<u8>
}

impl Pybadge {
	fn init(port: SerialPortInfo) -> anyhow::Result<Self> {
		let port_name = port.port_name.clone();
		let port = serialport::new(port.port_name, 960)
			.timeout(Duration::from_secs(1))
			.open()
			.with_context(|| "Failed to open port")?;
		Ok(Pybadge {
			port,
			port_name,
			buf: Vec::new()
		})
	}

	fn try_next_event(&mut self) -> Option<MessageToPc> {
		match bincode::decode_from_slice(&self.buf, bincode::config::standard()) {
			Ok((event, len)) => {
				self.buf.drain(..len);
				if event == MessageToPc::KeepAlive {
					//do not spam debug log full
					trace!("recieve message form {:?} {event:?}", self.port_name);
				} else {
					debug!("recieve message form {:?} {event:?}", self.port_name);
				};
				return Some(event);
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
		let len = match self.port.read(&mut buffer) {
			Ok(value) => value,
			Err(err) => match err.kind() {
				io::ErrorKind::TimedOut => 0,
				_ => panic!("{err}")
			}
		};
		if len != 0 {
			let mut new_data: Vec<u8> =
				buffer[..len].iter().map(|f| f.to_owned()).collect();
			trace!("recieve  data    form {:?} {new_data:?}", self.port_name);
			self.buf.append(&mut new_data);
		}
		None
	}

	fn write(&mut self, message: &MessageToPyBadge) {
		debug!("send message to {:?} {:?}", &self.port_name, &message);
		let data = bincode::encode_to_vec(message, bincode::config::standard()).unwrap();
		trace!("send  data   to {:?} {:?}", &self.port_name, data);
		self.port.write_all(&data).unwrap();
	}
}
