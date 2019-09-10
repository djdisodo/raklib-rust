extern crate rust_sort;

use crate::protocol::packet::{Packet, Encode};
use self::rust_sort::quick_sort::quick_sort;
use binaryutils::binary::Endian::{Little, Big};
use binaryutils::binary::write_unsigned_triad;
use std::ops::{Deref, DerefMut};

pub struct AcknowledgePacket {
	packet : Packet,
	packets : Vec<u32>
}

impl AcknowledgePacket {
	const RECORD_TYPE_RANGE : u8 = 0x00; //0
	const RECORD_TYPE_SINGLE : u8 = 0x01; //1
	pub fn new(buffer : Vec<u8>, offset : usize) -> AcknowledgePacket {
		return AcknowledgePacket {
			packet : Packet::new(buffer, offset),
			packets : Vec::new()
		};
	}
	fn clean(&mut self) {
		self.packets.clear();
		self.packet.clean();
	}
}

impl Deref for AcknowledgePacket {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.packet;
	}
}

impl DerefMut for AcknowledgePacket {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.packet;
	}
}

impl Encode for AcknowledgePacket {
	fn encode_header(&mut self) {
		self.packet.encode_header();
	}

	fn encode_payload(&mut self) {
		let mut payload : Vec<u8> = Vec::new();
		quick_sort(self.packets.as_mut_slice());
		let count : usize = self.packets.len();
		let mut records : u16 = 0;
		if count > 0 {
			let mut pointer : usize = 1;
			let mut start: u32 = self.packets.get(0).unwrap().clone();
			let mut last : u32 = start.clone();
			let mut current : u32;
			let mut diff : i64;
			while pointer < count {
				current = self.packets.get(pointer).unwrap().clone();
				pointer += 1;
				diff = (current - last) as i64;
				if diff == 1 {
					last = current;
				} else if diff > 1 {
					if start == last {
						payload.push(Self::RECORD_TYPE_RANGE as u8);
						payload.extend(write_unsigned_triad(start, Little));
						last = current;
						start = last;
					} else {
						payload.push(Self::RECORD_TYPE_RANGE as u8);
						payload.extend(write_unsigned_triad(start, Little));
						payload.extend(write_unsigned_triad(last, Little));
						last = current;
						start = last
					}
					records += 1;
				}
			}
			if start == last {
				payload.push(Self::RECORD_TYPE_RANGE as u8);
				payload.extend(write_unsigned_triad(start, Little));
			} else {
				payload.push(Self::RECORD_TYPE_RANGE as u8);
				payload.extend(write_unsigned_triad(start, Little));
				payload.extend(write_unsigned_triad(last, Little));
			}
			records += 1;
		}
		self.put_unsigned_short(records, Big);
		self.put(payload);
	}

	fn decode_header(&mut self) {
		self.packet.decode_header();
	}

	fn decode_payload(&mut self) {
		let count : u16 = self.get_unsigned_short(Big);
		self.packets.clear();
		let mut cnt : usize = 0;
		for _i in 0..count {
			if self.get_byte() == Self::RECORD_TYPE_RANGE {
				let start : u32 = self.get_unsigned_triad(Little);
				let mut end : u32 = self.get_unsigned_triad(Little);
				if (end - start) > 512 {
					end = start + 512;
				}
				for _c in start..(end + 1) {
					*self.packets.get_mut(cnt).unwrap() = self.get_unsigned_triad(Little);
					cnt += 1;
				}
			}
			if self.feof() || cnt < 4096 {
				break;
			}
		}
	}
}
