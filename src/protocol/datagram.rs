use crate::protocol::encapsulated_packet::EncapsulatedPacket;
use crate::protocol::packet::{Packet, Encode};
use std::ops::{Deref, DerefMut};
use crate::protocol::message_identifiers::ID_UNDEFINED;
use binaryutils::binary::Endian::Little;

pub struct Datagram {
	parent : Packet,
	pub header_flags : u8,
	pub packets : Vec<EncapsulatedPacket>,
	pub seq_number : /* Triad */u32
}

impl Datagram {
	pub const BITFLAG_VALID : u8 = 0x80;
	pub const BITFLAG_ACK : u8 = 0x40;

	/*
	 * These flags can be set on regular datagrams, but they are useless as per the public version of RakNet
	 * (the receiving client will not use them or pay any attention to them).
	 */
	pub const BITFLAG_PACKET_PAIR : u8 = 0x10;
	pub const BITFLAG_CONTINUOUS_SEND : u8 = 0x08;
	pub const BITFLAG_NEEDS_B_AND_AS : u8 = 0x04;

	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent: Packet::new(buffer, offset, Self::PACKET_ID),
			header_flags: 0,
			packets: Vec::new(),
			seq_number: 0
		}
	}
	pub fn length(&self) -> usize {
		let mut length : usize = 4;
		for i in 0..self.packets.len() {
			length += self.packets[i].get_total_length();
		}

		return length;
	}
	pub fn clean(&mut self) {
		self.packets.clear();
		self.seq_number = 0;
		self.parent.clean();
	}
}

impl Deref for Datagram {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for Datagram {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for Datagram {
	/* DON'T USE THIS VALUE */
	const PACKET_ID: u8 = ID_UNDEFINED;

	fn encode_clean(&mut self) {
		self.parent.encode_clean();
	}

	fn decode_clean(&mut self) {
		self.parent.decode_clean();
	}

	fn encode_header(&mut self) {
		let v : u8 = Self::BITFLAG_VALID | self.header_flags;
		self.put_byte(v);
	}

	fn encode_payload(&mut self) {
		let v : u32 = self.seq_number;
		self.put_unsigned_triad(v, Little);
		for i in 0..self.packets.len() {
			let packet : Vec<u8> = (&self.packets[i]).into();
			self.put(packet);
		}
	}

	fn decode_header(&mut self) {
		self.header_flags = self.get_byte();
	}

	fn decode_payload(&mut self) {
		self.seq_number = self.get_unsigned_triad(Little);

		while !self.feof() {
			let encapsulated_packet : EncapsulatedPacket = EncapsulatedPacket::from(self.parent.deref_mut());
			self.packets.push(encapsulated_packet);
		}
	}
}