use crate::protocol::packet::{Packet, PacketTrait};
use binaryutils::binary_stream::BinaryStreamTrait;

pub struct AdvertiseSystem {
	pub packet : Packet,
	pub server_name : String
}

impl AdvertiseSystem {
	pub fn new(buffer : Vec<u8>, offset : usize) -> AdvertiseSystem {
		return AdvertiseSystem {
			packet : Packet::new(buffer, offset),
			server_name : String::new()
		}
	}
}

trait AdvertiseSystemTrait : PacketTrait {
	fn get_advertise_system_ref(&self) -> &AdvertiseSystem;
	fn get_advertise_system_mut(&mut self) -> &mut AdvertiseSystem;
	fn get_packet_ref(&self) -> &Packet {
		return &self.get_advertise_system_ref().packet;
	}

	fn get_packet_mut(&mut self) -> &mut Packet {
		return &mut self.get_advertise_system_mut().packet;
	}
	fn encode_payload(&mut self) {
		self.put_string(self.get_advertise_system_ref().server_name.as_str());
	}

	fn decode_payload(&mut self) {
		self.get_advertise_system_mut().server_name = String::from(self.get_string());
	}
}

impl AdvertiseSystemTrait for AdvertiseSystem {
	fn get_advertise_system_ref(&self) -> &AdvertiseSystem {
		return self;
	}

	fn get_advertise_system_mut(&mut self) -> &mut AdvertiseSystem {
		return self;
	}
}
impl PacketTrait for AdvertiseSystem {
	const PACKET_ID: u8 = unimplemented!();

	fn get_packet_ref(&self) -> &Packet {
		unimplemented!()
	}

	fn get_packet_mut(&mut self) -> &mut Packet {
		unimplemented!()
	}

	fn encode_payload(&mut self) {
		unimplemented!()
	}

	fn decode_payload(&mut self) {
		unimplemented!()
	}
}

impl BinaryStreamTrait for AdvertiseSystem {}