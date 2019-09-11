use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_CONNECTED_PING;
use std::ops::{Deref, DerefMut};
use binaryutils::binary::Endian::Big;

pub struct ConnectedPing {
	pub packet : Packet,
	pub send_ping_time : u64
}

impl ConnectedPing {
	pub fn new(buffer : Vec<u8>, offset : usize) -> ConnectedPing {
		return ConnectedPing {
			packet : Packet::new(buffer, offset, Self::PACKET_ID),
			send_ping_time : 0
		}
	}
}

impl Deref for ConnectedPing {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.packet;
	}
}

impl DerefMut for ConnectedPing {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.packet;
	}
}
impl Encode for ConnectedPing {
	const PACKET_ID: u8 = ID_CONNECTED_PING;

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
		let v : u64 = self.send_ping_time;
		self.put_unsigned_long(v, Big);
	}

	fn decode_header(&mut self) {
		self.packet.decode_header();
	}

	fn decode_payload(&mut self) {
		self.send_ping_time = self.get_unsigned_long(Big);
	}
}