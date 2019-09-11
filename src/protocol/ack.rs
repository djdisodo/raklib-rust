use crate::protocol::acknowledge_packet::AcknowledgePacket;
use crate::protocol::packet::Encode;
use std::ops::{Deref, DerefMut};

pub struct ACK {
	acknowledge_packet : AcknowledgePacket,
}

impl ACK {
	pub fn new(buffer : Vec<u8>, offset : usize) -> ACK {
		return ACK {
			acknowledge_packet : AcknowledgePacket::new(buffer, offset, Self::PACKET_ID)
		};
	}
}
impl Deref for ACK {
	type Target = AcknowledgePacket;

	fn deref(&self) -> &Self::Target {
		return &self.acknowledge_packet;
	}
}
impl DerefMut for ACK {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.acknowledge_packet;
	}
}

impl Encode for ACK {
	const PACKET_ID : u8 = 0xc0;
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
		self.acknowledge_packet.encode_payload();
	}

	fn decode_header(&mut self) {
		self.packet.decode_header();
	}

	fn decode_payload(&mut self) {
		self.acknowledge_packet.decode_payload();
	}
}