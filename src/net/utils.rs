use regex::Regex;
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
            if let IpAddr::V4(ip) = iface.addrs[0].addr {
                return Some(ip.octets());
            }
        }
    }

    None
}

pub fn is_valid(target_a_ip: &str, target_b_ip: &str) -> Vec<String> {
    let mut invalid_ips = Vec::new();
    let valid_ip = Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();

    if !valid_ip.is_match(target_a_ip) {
        invalid_ips.push(target_a_ip.to_owned());
    }
    if !valid_ip.is_match(target_b_ip) {
        invalid_ips.push(target_b_ip.to_owned());
    }

    invalid_ips
}

pub fn parse_ip(ip: &str) -> [u8; 4] {
    let mut bytes = [0; 4];

    for (i, byte) in ip.split('.').enumerate() {
        bytes[i] = byte.parse().unwrap();
    }

    bytes
}

pub fn get_local_mac(iface: String) -> [u8; 6] {
    let mut bytes = [0; 6];
    let mut mac = String::new();
    let mut file = File::open(format!("/sys/class/net/{}/address", iface)).unwrap();
    file.read_to_string(&mut mac).unwrap();

    for (i, byte) in mac.trim().split(':').enumerate() {
        bytes[i] = u8::from_str_radix(byte, 16).unwrap();
    }

    bytes
}

pub fn build_socket() -> Result<RawSocket, String> {
    match get_iface_name() {
        Some(iface) => match RawSocket::new(iface.as_str()) {
            Ok(socket) => Ok(socket),
            Err(e) => Err(e.to_string()),
        },
        None => Err("no network interface found".to_owned()),
    }
}
