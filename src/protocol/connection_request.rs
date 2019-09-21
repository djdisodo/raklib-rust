use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_CONNECTION_REQUEST;
use std::ops::{Deref, DerefMut};
use binaryutils::binary::Endian::Big;

pub struct ConnectionRequest {
	parent : Packet,
	pub client_id : u64,
	pub send_ping_time : u64,
	pub use_security : bool
}

impl ConnectionRequest {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : Packet::new(buffer, offset, Self::PACKET_ID),
			client_id : 0,
			send_ping_time : 0,
			use_security : false
		}
	}
}

impl Deref for ConnectionRequest {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for ConnectionRequest {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for ConnectionRequest {
	const PACKET_ID: u8 = ID_CONNECTION_REQUEST;

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
		let v : u64 = self.client_id;
		self.put_unsigned_long(v, Big);
		let v : u64 = self.send_ping_time;
		self.put_unsigned_long(v, Big);
		let v : bool = self.use_security;
		self.put_bool(v); //diff
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn decode_payload(&mut self) {
		self.client_id = self.get_unsigned_long(Big);
		self.send_ping_time = self.get_unsigned_long(Big);
		self.use_security = self.get_bool(); //diff
	}
}