use crate::protocol::packet::Packet;

pub struct InternetAddress {
	ip : String,
	port : u16,
	version : u8
}
impl InternetAddress {
	pub fn new(address : String, port : u16, version : u8) -> Self{
		return InternetAddress {
			ip : address,
			port,
			version
		};
	}
	pub fn get_ip(&self) -> &String {
		return &self.ip;
	}
	pub fn get_port(&self) -> u16 {
		return self.port;
	}
	pub fn get_version(&self) -> u8 {
		return self.version;
	}
	pub fn equals(&self, address : InternetAddress) -> bool {
		return self.get_ip() == address.get_ip() && self.get_port() == address.get_port() && self.get_version() == address.get_version();
	}
}
