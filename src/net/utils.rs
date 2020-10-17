use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::str::FromStr;
use crate::mac_addr::MacAddr;

pub fn get_local_mac(iface: &str) -> MacAddr {
    let mut addr = String::new();
    let mut file = File::open(format!("/sys/class/net/{}/address", iface)).unwrap();
    file.read_to_string(&mut addr).unwrap();

    MacAddr::from_str(&addr).unwrap()
}

pub fn get_local_ip() -> Option<Ipv4Addr> {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap();
    if let IpAddr::V4(addr) = socket.local_addr().unwrap().ip() {
        Some(addr)
    } else {
        None
    }
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
