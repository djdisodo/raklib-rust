use crate::protocol::packet::Encode;
use crate::protocol::message_identifiers::ID_UNCONNECTED_PING;
use std::ops::{Deref, DerefMut};
use crate::protocol::offline_message::OfflineMessage;
use binaryutils::binary::Endian::Big;

pub struct UnconnectedPing {
	parent : OfflineMessage,
	pub send_ping_time : u64,
	pub client_id : u64
}

impl UnconnectedPing {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : OfflineMessage::new(buffer, offset, Self::PACKET_ID),
			send_ping_time: 0,
			client_id: 0
		}
	}
}

impl Deref for UnconnectedPing {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for UnconnectedPing {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for UnconnectedPing {
	const PACKET_ID: u8 = ID_UNCONNECTED_PING;

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
		self.write_magic();
		let client_id : u64 = self.client_id;
		self.put_unsigned_long(client_id, Big);
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn decode_payload(&mut self) {
		self.send_ping_time = self.get_unsigned_long(Big);
		self.read_magic();
		self.client_id = self.get_unsigned_long(Big);
	}
}