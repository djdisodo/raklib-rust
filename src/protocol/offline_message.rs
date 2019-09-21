use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_UNDEFINED;
use std::ops::{Deref, DerefMut};

pub struct OfflineMessage {
	parent : Packet,
	pub magic : Vec<u8>
}

impl OfflineMessage {
	const MAGIC : [u8; 16] = [0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78];
	pub fn new(buffer : Vec<u8>, offset : usize, id : u8) -> Self {
		return Self {
			parent : Packet::new(buffer, offset, id),
			magic: Vec::new()
		}
	}

	pub fn read_magic(&mut self) {
		self.magic = self.get(16);
	}

	pub fn write_magic(&mut self) {
		self.put(Self::MAGIC.to_vec());
	}

	pub fn is_valid(&self) -> bool {
		return self.magic == Self::MAGIC;
	}
}

impl Deref for OfflineMessage {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for OfflineMessage {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for OfflineMessage {
	const PACKET_ID: u8 = ID_UNDEFINED;

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