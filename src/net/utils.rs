use smoltcp::phy::RawSocket;
use std::fs::File;
use std::io::Read;
use systemstat::data::IpAddr;
use systemstat::{Platform, System};

pub fn get_iface_name() -> Option<String> {
	for iface in System::new().networks().unwrap().values() {
		if iface.name != "lo" {
			return Some(iface.name.clone());
		}
	}

	None
}

pub fn get_local_ip() -> Option<[u8; 4]> {
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

pub fn parse_ip(ip: String) -> [u8; 4] {
	let mut bytes = [0; 4];

	for (i, byte) in ip.split(".").enumerate() {
		bytes[i] = byte.parse().unwrap();
	}

	bytes
}

pub fn get_local_mac(iface: String) -> [u8; 6] {
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
