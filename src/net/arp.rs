use crate::net::utils::*;
use smoltcp::phy::{wait, Device, RawSocket, RxToken, TxToken};
use smoltcp::time::Instant;
use std::os::unix::io::AsRawFd;

fn craft_request(buffer: &mut [u8], target_ip: &[u8]) -> Result<(), smoltcp::Error> {
	let sender_mac = get_local_mac(get_iface_name().unwrap());
	let sender_ip = get_local_ip().unwrap();
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

	// target_ip MAC address
	for i in 32..38 {
		buffer[i] = 0;
	}

	// target_ip IP address
	j = 0;
	for i in 38..42 {
		buffer[i] = target_ip[j];
		j += 1;
	}

	Ok(())
}

pub fn send_request(socket: &mut RawSocket, target_ip: &[u8]) {
	let tx = socket.transmit().unwrap();
	tx.consume(Instant::now(), 42, |buffer| {
		craft_request(buffer, target_ip)
	})
	.unwrap();
}

fn filter_reply(buffer: &mut [u8], target_ip: &[u8]) -> bool {
	let local_mac = get_local_mac(get_iface_name().unwrap());
	let mut j = 0;
	let mut ip_ok = true;
	let mut mac_ok = true;

	// dest == local_mac
	for i in 0..6 {
		if buffer[i] != local_mac[j] {
			mac_ok = false;
			break;
		} else {
			j += 1;
		}
	}

	// target_ip == sender_ip
	j = 0;
	for i in 28..32 {
		if buffer[i] != target_ip[j] {
			ip_ok = false;
			break;
		} else {
			j += 1;
		}
	}

	buffer[12] == 8 && buffer[13] == 6 && buffer[20] == 0 && buffer[21] == 2 && ip_ok && mac_ok
}

fn handle_reply(buffer: &mut [u8], target_ip: &[u8]) -> Result<[u8; 6], smoltcp::Error> {
	if filter_reply(buffer, target_ip) {
		println!("{:x?}\n", buffer);

		// extract target's MAC address
		let mut j = 0;
		let mut mac = [0; 6];

		for i in 6..12 {
			mac[j] = buffer[i];
			j += 1;
		}

		return Ok(mac);
	}

	Err(smoltcp::Error::__Nonexhaustive)
}

pub fn capture_reply(socket: &mut RawSocket, target_ip: &[u8]) -> [u8; 6] {
	loop {
		wait(socket.as_raw_fd(), None).unwrap();
		let (rx, _) = socket.receive().unwrap();
		match rx.consume(Instant::now(), |buffer| handle_reply(buffer, target_ip)) {
			Ok(mac) => return mac,
			Err(_) => println!("not yet"),
		}
	}
}
