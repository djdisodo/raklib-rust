use crate::protocol::packet::Encode;
use crate::protocol::message_identifiers::ID_OPEN_CONNECTION_REPLY_2;
use std::ops::{Deref, DerefMut};
use crate::protocol::offline_message::OfflineMessage;
use binaryutils::binary::Endian::Big;
use crate::utils::internet_address::InternetAddress;

pub struct OpenConnectionReply2 {
	parent : OfflineMessage,
	pub server_id : u64,
	pub client_address : InternetAddress,
	pub mtu_size : u16,
	pub server_security : bool
}

impl OpenConnectionReply2 {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : OfflineMessage::new(buffer, offset),
			server_id : 0,
			client_address: InternetAddress::dummy(),
			mtu_size: 0,
			server_security: false
		}
	}
}

impl Deref for OpenConnectionReply2 {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for OpenConnectionReply2 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for OpenConnectionReply2 {
	const PACKET_ID: u8 = ID_OPEN_CONNECTION_REPLY_2;

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
		self.write_magic();
		let server_id : u64 = self.server_id;
		self.put_unsigned_long(server_id, Big);
		let client_address : InternetAddress = self.client_address.clone();
		self.put_address(client_address);
		let mtu_size : u16 = self.mtu_size;
		self.put_unsigned_short(mtu_size, Big);
		let server_security : bool = self.server_security;
		self.put_bool(server_security);
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn decode_payload(&mut self) {
		self.read_magic();
		self.server_id = self.get_unsigned_long(Big);
		self.client_address = self.get_address();
		self.mtu_size = self.get_unsigned_short(Big);
		self.server_security = self.get_bool();
	}
}