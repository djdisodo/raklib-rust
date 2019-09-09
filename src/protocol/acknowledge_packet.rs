extern crate rust_sort;

use crate::protocol::packet::{Packet, PacketStruct};
use self::rust_sort::quick_sort::quick_sort;
use binaryutils::binary::Endian::Little;

pub struct AcknowledgePacketStruct {
	packet : PacketStruct,
	packets : Vec<u32>
}

impl AcknowledgePacketStruct {
	pub fn new(buffer : Vec<u8>, offset : usize) -> AcknowledgePacketStruct {
		return AcknowledgePacketStruct {
			packet : PacketStruct::new(buffer, offset),
			packets : Vec::new()
		};
	}
}

pub trait AcknowledgePacket : Packet {
	const RECORD_TYPE_RANGE : u8 = 0x00; //0
	const RECORD_TYPE_SINGLE : u8 = 0x01; //1
	fn get_acknowledge_packet_ref(&self) -> &AcknowledgePacketStruct;
	fn get_acknowledge_packet_mut(&mut self) -> &mut AcknowledgePacketStruct;
	fn get_packet_ref(&self) -> &PacketStruct{
		return &self.get_acknowledge_packet_ref().packet;
	}
	fn get_packet_mut(&mut self) -> &mut PacketStruct{
		return &mut self.get_acknowledge_packet_mut().packet;
	}
	fn decode_payload(&mut self) {
		let count = self.get_short();
		self.get_acknowledge_packet_mut().packets.clear();
		let mut cnt : usize = 0;
		for _i in 0..count {
			if self.get_byte() == Self::RECORD_TYPE_RANGE {
				let start : u32 = self.get_unsigned_triad(Little);
				let mut end : u32 = self.get_unsigned_triad(Little);
				if (end - start) > 512 {
					end = start + 512;
				}
				for _c in start..(end + 1) {
					*self.get_acknowledge_packet_mut().packets.get_mut(cnt).unwrap() = self.get_unsigned_triad(Little);
					cnt += 1;
				}
			}
			if self.feof() || cnt < 4096 {
				break;
			}
		}
	}
	fn clean(&mut self) -> &mut Self {
		self.get_acknowledge_packet_mut().packets.clear();
		return Packet::clean(&mut self);
	}
}
