use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_NEW_INCOMING_CONNECTION;
use std::ops::{Deref, DerefMut};
use binaryutils::binary::Endian::Big;
use crate::utils::internet_address::InternetAddress;
use crate::rnet::SYSTEM_ADDRESS_COUNT;

pub struct NewIncomingConnection {
	parent : Packet,
	pub address : InternetAddress,
	pub system_addresses : Vec<InternetAddress>,
	pub send_ping_time : u64,
	pub send_pong_time : u64
}

impl NewIncomingConnection {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : Packet::new(buffer, offset, Self::PACKET_ID),
			address : InternetAddress::dummy(),
			system_addresses : Vec::new(),
			send_ping_time : 0,
			send_pong_time : 0
		}
	}
}

impl Deref for NewIncomingConnection {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for NewIncomingConnection {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for NewIncomingConnection {
	const PACKET_ID: u8 = ID_NEW_INCOMING_CONNECTION;

	fn encode_clean(&mut self) {
		self.parent.encode_clean();
	}

	fn decode_clean(&mut self) {
		self.parent.decode_clean();
	}

	fn encode_header(&mut self) {
		self.parent.encode_header();
	}

	fn encode_payload(&mut self) {
		let address : InternetAddress = self.address.clone();
		self.put_address(address);
		for i in 0..self.system_addresses.len() {
			let address : InternetAddress = self.system_addresses[i].clone();
			self.put_address(address);
		}
		let send_ping_time : u64 = self.send_ping_time;
		self.put_unsigned_long(send_ping_time, Big);
		let send_pong_time : u64 = self.send_pong_time;
		self.put_unsigned_long(send_pong_time, Big);
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn decode_payload(&mut self) {
		self.address = self.get_address();

		//TODO: HACK!
		let stop_offset : usize = self.get_buffer().len() - 16; //buffer length - sizeof(sendPingTime) - sizeof(sendPongTime)
		let dummy : InternetAddress = InternetAddress::dummy();
		for i in 0..SYSTEM_ADDRESS_COUNT {
			if self.get_offset() >= stop_offset {
				self.system_addresses[i as usize] = dummy.clone();
			} else {
				self.system_addresses[i as usize] = self.get_address();
			}
		}

		self.send_ping_time = self.get_unsigned_long(Big);
		self.send_pong_time = self.get_unsigned_long(Big);
	}
}