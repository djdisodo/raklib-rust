use std::net::UdpSocket;
use crate::utils::internet_address::InternetAddress;
use std::io::Error;
use std::convert::TryInto;
use std::borrow::Borrow;

pub struct Socket {
	socket : UdpSocket,
	bind_address: InternetAddress
}

impl Socket {
	pub fn new(bind_address : InternetAddress) -> Result<Socket, Error> {
		let udp_socket : UdpSocket = UdpSocket::bind(bind_address.get_ip()).unwrap();
		if udp_socket.take_error().unwrap().is_some() {
			return Err(udp_socket.take_error().unwrap().unwrap());
		}
		let socket = Socket {
			socket : udp_socket,
			bind_address
		};
		socket.get_socket().set_nonblocking(true);
		return Ok(socket);
	}
	pub fn get_bind_address(&self) -> &InternetAddress {
		return &self.bind_address;
	}
	pub fn get_socket(&self) -> &UdpSocket {
		return &self.socket;
	}
	pub fn close(&mut self) {
		self.socket.connect("0.0.0.0");
	}
	pub fn get_last_error(&self) -> Option<Error> {
		return self.get_socket().take_error().unwrap();
	}
	pub fn read_packet(&self/* , source : &str, port : &u16 */) -> Option<Vec<u8>> {
		let mut buffer : Vec<u8> = Vec::new();
		let result = self.get_socket().recv(&mut buffer);
		if buffer.is_empty() {
			return None;
		} else {
			return Some(buffer);
		}
	}
	pub fn write_packet(&self, buffer : &str, dest : &str, port : &u8) -> Result<usize, Error>{
		return self.get_socket().send(buffer.as_bytes());
	}
}