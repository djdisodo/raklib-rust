use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_DISCONNECTION_NOTIFICATION;
use std::ops::{Deref, DerefMut};

pub struct DisconnectNotification {
	parent : Packet
}

impl DisconnectNotification {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : Packet::new(buffer, offset, Self::PACKET_ID)
		}
	}
}

impl Deref for DisconnectNotification {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for DisconnectNotification {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for DisconnectNotification {
	const PACKET_ID: u8 = ID_DISCONNECTION_NOTIFICATION;

	fn encode_clean(&mut self) {
		self.parent.encode_clean();
	}

	fn decode_clean(&mut self) {
		self.parent.decode_clean();
	}

	fn encode_header(&mut self) {
		self.parent.encode_header();
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}
}