use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_CONNECTION_REQUEST_ACCEPTED;
use std::ops::{Deref, DerefMut};
use binaryutils::binary::Endian::Big;
use crate::utils::internet_address::InternetAddress;
use crate::rnet::SYSTEM_ADDRESS_COUNT;

pub struct ConnectionRequestAccepted {
	pub packet : Packet,
	pub address : InternetAddress,
	pub system_addresses : Vec<InternetAddress>,
	pub send_ping_time : u64,
	pub send_pong_time : u64
}

impl ConnectionRequestAccepted {
	pub fn new(buffer : Vec<u8>, offset : usize) -> ConnectionRequestAccepted {
		return ConnectionRequestAccepted {
			packet : Packet::new(buffer, offset, Self::PACKET_ID),
			address : InternetAddress::new(String::from(""), 0, 0),
			system_addresses : Vec::new(),
			send_ping_time : 0,
			send_pong_time : 0
		}
	}
}

impl Deref for ConnectionRequestAccepted {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.packet;
	}
}

impl DerefMut for ConnectionRequestAccepted {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.packet;
	}
}
impl Encode for ConnectionRequestAccepted {
	const PACKET_ID: u8 = ID_CONNECTION_REQUEST_ACCEPTED;

	fn encode(&mut self) {
		self.packet.encode();
		self.encode_header();
		self.encode_payload();
	}
	fn decode(&mut self) {
		self.packet.decode();
		self.decode_header();
		self.decode_payload();
	}
	fn encode_header(&mut self) {
		self.packet.encode_header();
	}

	fn encode_payload(&mut self) {
		let address : InternetAddress = self.address.clone();
		self.put_address(address);
		self.put_unsigned_short(0, Big);
		let dummy = InternetAddress::new(String::from("0.0.0.0"), 0, 4);
		for i in 0..SYSTEM_ADDRESS_COUNT {
			let address : InternetAddress = self.system_addresses.get(i as usize).unwrap_or(&dummy.clone()).clone();
			self.put_address(address);
		}
		let v : u64 = self.send_ping_time;
		self.put_unsigned_long(v, Big);
		let v : u64 = self.send_pong_time;
		self.put_unsigned_long(v, Big);
	}

	fn decode_header(&mut self) {
		self.packet.decode_header();
	}

	fn decode_payload(&mut self) {
		self.address = self.get_address();
		drop(self.get_short(Big)/* TODO : check this */); //dropping to avoid some build warns
		let len : usize = self.get_buffer().len();
		let dummy = InternetAddress::new(String::from("0.0.0.0"), 0, 4);
		for i in 0..SYSTEM_ADDRESS_COUNT {
			self.system_addresses[i as usize] = if self.get_offset() + 16 < len { self.get_address() } else { dummy.clone() };
		}

		self.send_ping_time = self.get_unsigned_long(Big);
		self.send_pong_time = self.get_unsigned_long(Big);
	}
}