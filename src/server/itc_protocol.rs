/*
	 * These internal "packets" DO NOT exist in the RakNet protocol. They are used by the RakLib API to communicate
	 * messages between the RakLib thread and the implementation's thread.
	 *
	 * Internal Packet:
	 * byte (packet ID)
	 * byte[] (payload)
	 */

/*
 * ENCAPSULATED payload:
 * int32 (internal session ID)
 * byte (flags, last 3 bits, priority)
 * payload (binary internal EncapsulatedPacket)
 */
pub const PACKET_ENCAPSULATED : u8 = 0x01;

/*
 * OPEN_SESSION payload:
 * int32 (internal session ID)
 * byte (address length)
 * byte[] (address)
 * short (port)
 * long (clientID)
 */
pub const PACKET_OPEN_SESSION : u8 = 0x02;

/*
 * CLOSE_SESSION payload:
 * int32 (internal session ID)
 * string (reason)
 */
pub const PACKET_CLOSE_SESSION : u8 = 0x03;

/*
 * INVALID_SESSION payload:
 * int32 (internal session ID)
 */
pub const PACKET_INVALID_SESSION : u8 = 0x04;

/* TODO: implement this
 * SEND_QUEUE payload:
 * int32 (internal session ID)
 */
pub const PACKET_SEND_QUEUE : u8 = 0x05;

/*
 * ACK_NOTIFICATION payload:
 * int32 (internal session ID)
 * int32 (identifierACK)
 */
pub const PACKET_ACK_NOTIFICATION : u8 = 0x06;

/*
 * SET_OPTION payload:
 * byte (option name length)
 * byte[] (option name)
 * byte[] (option value)
 */
pub const PACKET_SET_OPTION : u8 = 0x07;

/*
 * RAW payload:
 * byte (address length)
 * byte[] (address from/to)
 * short (port)
 * byte[] (payload)
 */
pub const PACKET_RAW : u8 = 0x08;

/*
 * BLOCK_ADDRESS payload:
 * byte (address length)
 * byte[] (address)
 * int (timeout)
 */
pub const PACKET_BLOCK_ADDRESS : u8 = 0x09;

/*
 * UNBLOCK_ADDRESS payload:
 * byte (address length)
 * byte[] (address)
 */
pub const PACKET_UNBLOCK_ADDRESS : u8 = 0x10;

/*
 * REPORT_PING payload:
 * int32 (internal session ID)
 * int32 (measured latency in MS)
 */
pub const PACKET_REPORT_PING : u8 = 0x11;

/*
 * RAW_FILTER payload:
 * byte[] (pattern)
 */
pub const PACKET_RAW_FILTER : u8 = 0x12;

/*
 * No payload
 *
 * Sends the disconnect message, removes sessions correctly, closes sockets.
 */
pub const PACKET_SHUTDOWN : u8 = 0x7e;

/*
 * No payload
 *
 * Leaves everything as-is and halts, other Threads can be in a post-crash condition.
 */
pub const PACKET_EMERGENCY_SHUTDOWN : u8 = 0x7f;