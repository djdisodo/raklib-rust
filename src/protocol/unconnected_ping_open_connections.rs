use crate::protocol::packet::Encode;
use crate::protocol::message_identifiers::ID_UNCONNECTED_PING_OPEN_CONNECTIONS;
use std::ops::{Deref, DerefMut};
use crate::protocol::unconnected_ping::UnconnectedPing;

pub struct UnconnectedPingOpenConnections {
	parent : UnconnectedPing
}

impl UnconnectedPingOpenConnections {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : UnconnectedPing::new(buffer, offset)
		}
	}
}

impl Deref for UnconnectedPingOpenConnections {
	type Target = UnconnectedPing;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for UnconnectedPingOpenConnections {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for UnconnectedPingOpenConnections {
	const PACKET_ID: u8 = ID_UNCONNECTED_PING_OPEN_CONNECTIONS;

	fn encode_clean(&mut self) {
		self.parent.encode_clean();
	}

	fn decode_clean(&mut self) {
		self.parent.decode_clean();
	}

	fn encode_header(&mut self) {
		self.put_byte(Self::PACKET_ID);
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn encode_payload(&mut self) {
		self.parent.encode_payload();
	}

	fn decode_payload(&mut self) {
		self.parent.decode_payload();
	}
}