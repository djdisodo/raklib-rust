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