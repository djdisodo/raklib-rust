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
		unimplemented!(); //TODO
	}
}