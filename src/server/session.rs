use crate::protocol::encapsulated_packet::EncapsulatedPacket;
use crate::server::session_manager::SessionManager;
use crate::utils::internet_address::InternetAddress;
use std::time::SystemTime;
use crate::protocol::datagram::Datagram;

pub struct Session {
	message_index : /* Triad */u32,

	send_ordered_index : Vec</* Triad */u32>,
	send_sequenced_index : Vec</* Triad */u32>,
	receive_ordered_index : Vec</* Triad */u32>,
	receive_sequenced_highest_index : Vec</* Triad */u32>,
	receive_ordered_packets : Vec<EncapsulatedPacket>,

	session_manager : SessionManager,

	/* TODO logger : Logger, */

	address : InternetAddress,

	state : u8,
	mtu_size : u16,
	id : u64,
	split_id : u16,

	send_seq_number : /* Triad */u32,

	last_update : SystemTime,
	disconnection_time : Option<SystemTime>,

	is_temporal : bool,

	packet_to_send : Vec<Datagram>,
	is_active : bool,

	ack_queue : Vec</* Triad */u32>,
	nack_queue : Vec</* Triad */u32>,

	recovery_queue : Vec<Datagram>,

	split_packets : Vec<Vec<EncapsulatedPacket>>,

	need_ack : Vec<Vec</* Triad */u32>>,

	send_queue : Datagram
}
