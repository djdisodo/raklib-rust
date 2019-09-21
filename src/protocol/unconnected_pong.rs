use crate::protocol::packet::Encode;
use crate::protocol::message_identifiers::ID_UNCONNECTED_PONG;
use std::ops::{Deref, DerefMut};
use crate::protocol::offline_message::OfflineMessage;
use binaryutils::binary::Endian::Big;

pub struct UnconnectedPong {
	parent : OfflineMessage,
	pub send_ping_time : u64,
	pub server_id : u64,
	pub server_name : String
}

impl UnconnectedPong {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : OfflineMessage::new(buffer, offset, Self::PACKET_ID),
			send_ping_time: 0,
			server_id: 0,
			server_name: String::new()
		}
	}
}

impl Deref for UnconnectedPong {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for UnconnectedPong {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for UnconnectedPong {
	const PACKET_ID: u8 = ID_UNCONNECTED_PONG;

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
		let send_ping_time : u64 = self.send_ping_time;
		self.put_unsigned_long(send_ping_time, Big);
		let server_id : u64 = self.server_id;
		self.put_unsigned_long(server_id, Big);
		self.write_magic();
		let server_name : String = self.server_name.clone();
		self.put_string(server_name);
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn decode_payload(&mut self) {
		self.send_ping_time = self.get_unsigned_long(Big);
		self.server_id = self.get_unsigned_long(Big);
		self.read_magic();
		self.server_name = self.get_string();
	}
}