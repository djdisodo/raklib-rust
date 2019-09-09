use crate::protocol::acknowledge_packet::{AcknowledgePacket, AcknowledgePacketTrait};
use crate::protocol::packet::{PacketTrait, Packet};
use binaryutils::binary_stream::{BinaryStreamTrait, BinaryStream};

pub struct ACK {
	acknowledge_packet : AcknowledgePacket,
}

pub trait ACKTrait : AcknowledgePacketTrait {
	const PACKET_ID : u8 = 0xc0;
	fn get_ack_ref(&self) -> &ACK;
	fn get_ack_mut(&mut self) -> &mut ACK;
}

impl ACK {
	pub fn new(buffer : Vec<u8>, offset : usize) -> ACK {
		return ACK {
			acknowledge_packet : AcknowledgePacket::new(buffer, offset)
		};
	}
}

impl ACKTrait for ACK {
	fn get_ack_ref(&self) -> &ACK {
		return &self;
	}
	fn get_ack_mut(&mut self) -> &mut ACK {
		return self;
	}
}
impl AcknowledgePacketTrait for ACK {
	fn get_acknowledge_packet_ref(&self) -> &AcknowledgePacket {
		return &self.acknowledge_packet;
	}

	fn get_acknowledge_packet_mut(&mut self) -> &mut AcknowledgePacket {
		return &mut self.acknowledge_packet;
	}
}
impl PacketTrait for ACK {
	const PACKET_ID: u8 = 0xc0;
	fn get_packet_ref(&self) -> &Packet {
		unimplemented!();
	}

	fn get_packet_mut(&mut self) -> &mut Packet {
		unimplemented!();
	}

	fn encode_payload(&mut self) {
		unimplemented!();
	}

	fn decode_payload(&mut self) {
		unimplemented!()
	}
}
impl BinaryStreamTrait for ACK {
	fn get_binary_stream_ref(&self) -> &BinaryStream {
		unimplemented!()
	}

	fn get_binary_stream_mut(&self) -> &mut BinaryStream {
		unimplemented!()
	}
}