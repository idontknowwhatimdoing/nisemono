// use regex::Regex;
// use smoltcp::socket::{IcmpPacketMetadata, IcmpSocket, IcmpSocketBuffer};
// use smoltcp::wire::IpAddress;
use smoltcp::phy::{wait, Device, RawSocket, RxToken, TxToken};
use smoltcp::time::Instant;
use std::env::args;
use std::fs::File;
use std::io::Read;
// use std::os::unix::io::AsRawFd;
use systemstat::data::IpAddr;
use systemstat::{Platform, System};

fn get_iface_name() -> Option<String> {
	let ifaces = System::new().networks().unwrap();
	for iface in ifaces.values() {
		if iface.name != "lo" {
			return Some(iface.name.clone());
		}
	}

	None
}

fn get_local_ip() -> Option<[u8; 4]> {
	for iface in System::new().networks().unwrap().values() {
		if iface.name != "lo" {
			match iface.addrs[0].addr {
				IpAddr::V4(ip) => return Some(ip.octets()),
				_ => {}
			}
		}
	}

	None
}

fn get_local_mac(iface: String) -> [u8; 6] {
	let mut bytes = [0; 6];
	let mut mac = String::new();
	let mut file = File::open(format!("/sys/class/net/{}/address", iface)).unwrap();
	file.read_to_string(&mut mac).unwrap();

	for (i, byte) in mac.trim().split(":").enumerate() {
		bytes[i] = u8::from_str_radix(byte, 16).unwrap();
	}

	bytes
}

pub fn build_socket() -> Result<RawSocket, String> {
	match get_iface_name() {
		Some(iface) => match RawSocket::new(iface.as_str()) {
			Ok(socket) => return Ok(socket),
			Err(_) => return Err("\nsocket creation error\n".to_owned()),
		},
		None => return Err("\nno network interface found\n".to_owned()),
	}
}

fn _is_arp(buffer: &mut [u8]) -> bool {
	buffer[12] == 8 && buffer[13] == 6
}

fn _handle_frame(buffer: &mut [u8]) -> Result<(), smoltcp::Error> {
	if _is_arp(buffer) {
		println!("{:x?}\n", buffer);
	}
	Ok(())
}

fn parse_ip(ip: String) -> [u8; 4] {
	let mut bytes = [0; 4];
	for (i, byte) in ip.split(".").enumerate() {
		bytes[i] = byte.parse().unwrap();
	}
	bytes
}

fn craft_arp_request(buffer: &mut [u8]) -> Result<(), smoltcp::Error> {
	let sender_mac = get_local_mac(get_iface_name().unwrap());
	let sender_ip = get_local_ip().unwrap();
	let target_ip = parse_ip(args().nth(1).unwrap());
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

pub fn send_arp_request(socket: &mut RawSocket) {
	let tx = socket.transmit().unwrap();
	tx.consume(Instant::now(), 42, |buffer| craft_arp_request(buffer))
		.unwrap();
}

// pub fn capture_frames(mut socket: RawSocket) {
// 	loop {
// 		wait(socket.as_raw_fd(), None).unwrap();
// 		let (rx, _) = socket.receive().unwrap();
// 		rx.consume(Instant::now(), |buffer| handle_frame(buffer))
// 			.unwrap();
// 	}
// }

// Given a network address, returns all addresses that could be in use
// pub fn bruteforce_addrs(netaddr: String) -> Result<Vec<String>, String> {
// 	let re = Regex::new(r"(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})/(\d+)").unwrap();
// 	if re.is_match(netaddr.as_str()) {
// 		let caps = re.captures(netaddr.as_str()).unwrap();
// 		let cidr: u32 = caps.get(5).unwrap().as_str().parse().unwrap();
// 		let mut nb_hosts: u32 = 0;
// 		for i in 0..32 - cidr {
// 			nb_hosts += 1 * 2u32.pow(i);
// 		}

// 		let mut addr_bytes: [u8; 4] = [
// 			caps.get(1).unwrap().as_str().parse().unwrap(),
// 			caps.get(2).unwrap().as_str().parse().unwrap(),
// 			caps.get(3).unwrap().as_str().parse().unwrap(),
// 			caps.get(4).unwrap().as_str().parse().unwrap(),
// 		];
// 		addr_bytes[3] += 1;
// 		let mut hosts = Vec::new();
// 		hosts.push(format!(
// 			"{}.{}.{}.{}",
// 			addr_bytes[0], addr_bytes[1], addr_bytes[2], addr_bytes[3]
// 		));
// 		for _ in 1..nb_hosts - 1 {
// 			if addr_bytes[3] < 255 {
// 				addr_bytes[3] += 1;
// 			} else {
// 				addr_bytes[3] = 0;
// 				if addr_bytes[2] < 255 {
// 					addr_bytes[2] += 1;
// 				} else {
// 					addr_bytes[2] = 0;
// 					addr_bytes[1] += 1;
// 				}
// 			}
// 			hosts.push(format!(
// 				"{}.{}.{}.{}",
// 				addr_bytes[0], addr_bytes[1], addr_bytes[2], addr_bytes[3]
// 			));
// 		}
// 		Ok(hosts)
// 	} else {
// 		Err("invalid network address ...".to_owned())
// 	}
// }

// pub fn scan_network(_hosts: Vec<String>) {
// 	let rx = IcmpSocketBuffer::new(vec![IcmpPacketMetadata::EMPTY], vec![0; 256]);
// 	let tx = IcmpSocketBuffer::new(vec![IcmpPacketMetadata::EMPTY], vec![0; 256]);
// 	let mut ping = IcmpSocket::new(rx, tx);
// 	ping.send(256, IpAddress::v4(192, 168, 0, 254)).unwrap();
// }
