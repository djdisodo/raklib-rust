use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_ADVERTISE_SYSTEM;
use std::ops::{Deref, DerefMut};
use std::borrow::Borrow;

pub struct AdvertiseSystem {
	pub packet : Packet,
	pub server_name : String
}

impl AdvertiseSystem {
	pub fn new(buffer : Vec<u8>, offset : usize) -> AdvertiseSystem {
		return AdvertiseSystem {
			packet : Packet::new(buffer, offset),
			server_name : String::new()
		}
	}
}

impl Deref for AdvertiseSystem {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.packet;
	}
}

impl DerefMut for AdvertiseSystem {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.packet;
	}
}
impl Encode for AdvertiseSystem {
	fn encode_header(&mut self) {
		self.packet.encode_header();
	}

	fn encode_payload(&mut self) {
		let v : String = self.server_name.clone();
		self.put_string(v.borrow());
	}

	fn decode_header(&mut self) {
		self.packet.decode_header();
	}

	fn decode_payload(&mut self) {
		self.server_name = String::from(self.packet.get_string());
	}
}