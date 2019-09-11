use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_CONNECTED_PONG;
use std::ops::{Deref, DerefMut};
use binaryutils::binary::Endian::Big;

pub struct ConnectedPong {
	pub packet : Packet,
	pub send_ping_time : u64,
	pub send_pong_time : u64
}

impl ConnectedPong {
	pub fn new(buffer : Vec<u8>, offset : usize) -> ConnectedPong {
		return ConnectedPong {
			packet : Packet::new(buffer, offset, Self::PACKET_ID),
			send_ping_time : 0,
			send_pong_time : 0
		}
	}
}

impl Deref for ConnectedPong {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.packet;
	}
}

impl DerefMut for ConnectedPong {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.packet;
	}
}
impl Encode for ConnectedPong {
	const PACKET_ID: u8 = ID_CONNECTED_PONG;

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
		let v: u64 = self.send_ping_time;
		self.put_unsigned_long(v, Big);
		let v : u64 = self.send_pong_time;
		self.put_unsigned_long(v, Big);
	}

	fn decode_header(&mut self) {
		self.packet.decode_header();
	}

	fn decode_payload(&mut self) {
		self.send_ping_time = self.get_unsigned_long(Big);
		self.send_pong_time = self.get_unsigned_long(Big);
	}
}