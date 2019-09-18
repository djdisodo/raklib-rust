use crate::protocol::encapsulated_packet::EncapsulatedPacket;
use crate::protocol::packet::{Packet, Encode};
use std::ops::{Deref, DerefMut};
use crate::protocol::message_identifiers::ID_CONNECTED_PING;
use binaryutils::binary::Endian::{Little, Big};
use binaryutils::binary_stream::BinaryStream;

pub struct Datagram {
	pub header_flags : u8,
	pub packets : Vec<EncapsulatedPacket>,
	pub seq_number : /* Triad */u32,
	packet : Packet
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

	pub fn new(buffer : Vec<u8>, offset : usize) -> Datagram {
		return Datagram {
			header_flags: 0,
			packets: Vec::new(),
			seq_number: 0,
			packet: Packet::new(buffer, offset, Self::PACKET_ID)
		}
	}
	pub fn length(&self) -> usize {
		let mut length : usize = 4;
		for i in 0..self.packets.len() {
			length += self.packets.get(i).unwrap().get_total_length();
		}

		return length;
	}
	pub fn clean(&mut self) {
		self.packets.clear();
		self.seq_number = 0;
		self.packet.clean();
	}
}

impl Deref for Datagram {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.packet;
	}
}

impl DerefMut for Datagram {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.packet;
	}
}
impl Encode for Datagram {
	/* DON'T USE THIS VALUE */
	const PACKET_ID: u8 = ID_CONNECTED_PING;

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
		let v : u8 = Self::BITFLAG_VALID | self.header_flags;
		self.put_byte(v);
	}

	fn encode_payload(&mut self) {
		let v : u32 = self.seq_number;
		self.put_unsigned_triad(v, Little);
		for i in 0..self.packets.len() {
			let packet : Vec<u8> = self.packets.get(i).unwrap().into();
			self.put(packet);
		}
	}

	fn decode_header(&mut self) {
		self.header_flags = self.get_byte();
	}

	fn decode_payload(&mut self) {
		self.seq_number = self.get_unsigned_triad(Little);

		while !self.feof() {
			self.packets.push(EncapsulatedPacket::from(self.packet.deref_mut()));
		}
	}
}