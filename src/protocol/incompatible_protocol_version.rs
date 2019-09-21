use crate::protocol::packet::Encode;
use crate::protocol::message_identifiers::ID_INCOMPATIBLE_PROTOCOL_VERSION;
use std::ops::{Deref, DerefMut};
use crate::protocol::offline_message::OfflineMessage;
use binaryutils::binary::Endian::Big;

pub struct IncompatibleProtocolVersion {
	parent : OfflineMessage,
	pub protocol_version : u8,
	pub server_id : u64,
}

impl IncompatibleProtocolVersion {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : OfflineMessage::new(buffer, offset),
			protocol_version : 0,
			server_id : 0,
		}
	}
}

impl Deref for IncompatibleProtocolVersion {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for IncompatibleProtocolVersion {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for IncompatibleProtocolVersion {
	const PACKET_ID: u8 = ID_INCOMPATIBLE_PROTOCOL_VERSION;

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
		let protocol_version : u8 = self.protocol_version;
		self.put_byte(protocol_version);
		self.write_magic();
		let server_id : u64 = self.server_id;
		self.put_unsigned_long(server_id, Big);
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn decode_payload(&mut self) {
		self.protocol_version = self.get_byte();
		self.read_magic();
		self.server_id = self.get_unsigned_long(Big);
	}
}