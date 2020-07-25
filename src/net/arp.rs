use crate::net::utils::*;
use smoltcp::phy::{wait, Device, RawSocket, RxToken, TxToken};
use smoltcp::time::Instant;
use std::env::args;
use std::os::unix::io::AsRawFd;

fn _filter_reply(buffer: &mut [u8]) -> bool {
	buffer[12] == 8 && buffer[13] == 6
}

fn _handle_reply(buffer: &mut [u8]) -> Result<(), smoltcp::Error> {
	if _filter_reply(buffer) {
		println!("{:x?}\n", buffer);
	}

	Ok(())
}

pub fn _capture_reply(socket: &mut RawSocket) {
	loop {
		wait(socket.as_raw_fd(), None).unwrap();
		let (rx, _) = socket.receive().unwrap();
		rx.consume(Instant::now(), |buffer| _handle_reply(buffer))
			.unwrap();
	}
}

fn craft_request(buffer: &mut [u8]) -> Result<(), smoltcp::Error> {
	let sender_mac = get_local_mac(get_iface_name().unwrap());
	let sender_ip = get_local_ip().unwrap();
	let target_ip = parse_ip(args().nth(1).unwrap()).unwrap();
	let mut j = 0;

	// dest
	for i in 0..6 {
		buffer[i] = 0xff;
	}

	// src
	for i in 6..12 {
		buffer[i] = sender_mac[j];
		j += 1;
	}

	// type
	buffer[12] = 8;
	buffer[13] = 6;

	// hardware type
	buffer[14] = 0;
	buffer[15] = 1;

	// protocol type
	buffer[16] = 8;
	buffer[17] = 0;

	// hardware size
	buffer[18] = 6;

	// protocol size
	buffer[19] = 4;

	// opcode
	buffer[20] = 0;
	buffer[21] = 1;

	// sender MAC address
	j = 0;
	for i in 22..28 {
		buffer[i] = sender_mac[j];
		j += 1;
	}

	// sender IP address
	j = 0;
	for i in 28..32 {
		buffer[i] = sender_ip[j];
		j += 1;
	}

	// target MAC address
	for i in 32..38 {
		buffer[i] = 0;
	}

	// target IP address
	j = 0;
	for i in 38..42 {
		buffer[i] = target_ip[j];
		j += 1;
	}

	Ok(())
}

pub fn send_request(socket: &mut RawSocket) {
	let tx = socket.transmit().unwrap();
	tx.consume(Instant::now(), 42, |buffer| craft_request(buffer))
		.unwrap();
}
