use crate::protocol::packet_reliability::PacketReliability::{Unreliable, UnreliableSequenced, Reliable, ReliableOrdered, ReliableSequenced, UnreliableWithACKReceipt, ReliableWithACKReceipt, ReliableOrderedWithACKReceipt};

pub enum PacketReliability {
	Unreliable,
	UnreliableSequenced,
	Reliable,
	ReliableOrdered,
	ReliableSequenced,
	UnreliableWithACKReceipt,
	ReliableWithACKReceipt,
	ReliableOrderedWithACKReceipt
}

impl PacketReliability {
	pub fn is_reliable(&self) -> bool {
		match self {
			PacketReliability::Reliable => true,
			PacketReliability::ReliableOrdered => true,
			PacketReliability::ReliableSequenced => true,
			PacketReliability::ReliableWithACKReceipt => true,
			PacketReliability::ReliableOrderedWithACKReceipt => true,
			_ => false
		}
	}
	pub fn is_sequenced(&self) -> bool {
		match self {
			PacketReliability::UnreliableSequenced => true,
			PacketReliability::ReliableSequenced => true,
			_ => false
		}
	}
	pub fn is_ordered(&self) -> bool {
		match self {
			PacketReliability::ReliableOrdered => true,
			PacketReliability::ReliableOrderedWithACKReceipt => true,
			_ => false
		}
	}
	pub fn is_sequence_or_ordered(&self) -> bool {
		return self.is_sequenced() || self.is_ordered();
	}
}

impl From<u8> for PacketReliability {
	fn from(v : u8) -> Self {
		match v {
			0x00 => Unreliable,
			0x01 => UnreliableSequenced,
			0x02 => Reliable,
			0x03 => ReliableOrdered,
			0x04 => ReliableSequenced,
			0x05 => UnreliableWithACKReceipt,
			0x06 => ReliableWithACKReceipt,
			0x07 => ReliableOrderedWithACKReceipt,
			_ => panic!("unknown packet reliablity")
		}
	}
}