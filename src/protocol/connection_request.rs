use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_CONNECTION_REQUEST;
use std::ops::{Deref, DerefMut};
use binaryutils::binary::Endian::Big;

pub struct ConnectionRequest {
	pub packet : Packet,
	pub client_id : u64,
	pub send_ping_time : u64,
	pub use_security : bool
}

impl ConnectionRequest {
	pub fn new(buffer : Vec<u8>, offset : usize) -> ConnectionRequest {
		return ConnectionRequest {
			packet : Packet::new(buffer, offset, Self::PACKET_ID),
			client_id : 0,
			send_ping_time : 0,
			use_security : false
		}
	}
}

impl Deref for ConnectionRequest {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.packet;
	}
}

impl DerefMut for ConnectionRequest {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.packet;
	}
}
impl Encode for ConnectionRequest {
	const PACKET_ID: u8 = ID_CONNECTION_REQUEST;

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
		let v : u64 = self.client_id;
		self.put_unsigned_long(v, Big);
		let v : u64 = self.send_ping_time;
		self.put_unsigned_long(v, Big);
		let v : bool = self.use_security;
		self.put_bool(v); //diff
	}

	fn decode_header(&mut self) {
		self.packet.decode_header();
	}

	fn decode_payload(&mut self) {
		self.client_id = self.get_unsigned_long(Big);
		self.send_ping_time = self.get_unsigned_long(Big);
		self.use_security = self.get_bool(); //diff
	}
}