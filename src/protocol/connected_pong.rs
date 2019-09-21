use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_CONNECTED_PONG;
use std::ops::{Deref, DerefMut};
use binaryutils::binary::Endian::Big;

pub struct ConnectedPong {
	parent : Packet,
	pub send_ping_time : u64,
	pub send_pong_time : u64
}

impl ConnectedPong {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : Packet::new(buffer, offset, Self::PACKET_ID),
			send_ping_time : 0,
			send_pong_time : 0
		}
	}
}

impl Deref for ConnectedPong {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for ConnectedPong {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for ConnectedPong {
	const PACKET_ID: u8 = ID_CONNECTED_PONG;

	fn encode_clean(&mut self) {
		self.parent.encode_clean();
	}

	fn decode_clean(&mut self) {
		self.parent.decode_clean();
	}

	fn encode_header(&mut self) {
		self.parent.encode_header();
	}

	fn encode_payload(&mut self) {
		let v: u64 = self.send_ping_time;
		self.put_unsigned_long(v, Big);
		let v : u64 = self.send_pong_time;
		self.put_unsigned_long(v, Big);
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn decode_payload(&mut self) {
		self.send_ping_time = self.get_unsigned_long(Big);
		self.send_pong_time = self.get_unsigned_long(Big);
	}
}