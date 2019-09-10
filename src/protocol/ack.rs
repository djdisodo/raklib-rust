use crate::protocol::acknowledge_packet::AcknowledgePacket;
use crate::protocol::packet::Packet;
use binaryutils::binary_stream::BinaryStream;
use std::ops::{Deref, DerefMut};

pub struct ACK {
	acknowledge_packet : AcknowledgePacket,
}

impl ACK {
	const PACKET_ID : u8 = 0xc0;
	pub fn new(buffer : Vec<u8>, offset : usize) -> ACK {
		return ACK {
			acknowledge_packet : AcknowledgePacket::new(buffer, offset)
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