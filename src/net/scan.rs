use crate::arp::request;
use regex::Regex;
use smoltcp::phy::{wait, Device, RawSocket, RxToken};
use std::os::unix::io::AsRawFd;
use std::time::{Duration, Instant};

// Given a network address, returns all addresses that could be in use
pub fn bruteforce_addrs(netaddr: &str) -> Result<Vec<[u8; 4]>, String> {
    let re = Regex::new(r"(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)/(\d{1,2})").unwrap();
    if re.is_match(netaddr) {
        let caps = re.captures(netaddr).unwrap();
        let cidr: u32 = caps.get(5).unwrap().as_str().parse().unwrap();

        let mut nb_hosts: u32 = 0;
        for i in 0..32 - cidr {
            nb_hosts += 2u32.pow(i);
        }

        let mut addr_bytes: [u8; 4] = [
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
        ];
        addr_bytes[3] += 1;

        let mut hosts = Vec::new();
        hosts.push(addr_bytes);
        for _ in 1..nb_hosts - 1 {
            if addr_bytes[3] < 255 {
                addr_bytes[3] += 1;
            } else {
                addr_bytes[3] = 0;
                if addr_bytes[2] < 255 {
                    addr_bytes[2] += 1;
                } else {
                    addr_bytes[2] = 0;
                    addr_bytes[1] += 1;
                }
            }
            hosts.push(addr_bytes);
        }

        Ok(hosts)
    } else {
        Err("invalid network address ...".to_owned())
    }
}

fn check_alive(socket: &mut RawSocket, host: &[u8]) -> Result<bool, smoltcp::Error> {
    loop {
        wait(socket.as_raw_fd(), None).unwrap();
        let (rx, _) = socket.receive().unwrap();
        rx.consume(smoltcp::time::Instant::now(), |buffer| {
            if request::is_target_reply(buffer, host) {
                return Ok(true);
            } else {
                return Err(smoltcp::Error::Unaddressable);
            }
        })
        .unwrap();
    }
}

pub fn get_alive_hosts(socket: &mut RawSocket, hosts: Vec<[u8; 4]>) -> Vec<[u8; 4]> {
    let mut alive_hosts = Vec::new();

    for host in hosts {
        request::send(socket, &host);
        let now = Instant::now();

        loop {
            match check_alive(socket, &host) {
                Ok(_) => alive_hosts.push(host),
                Err(_) => {
                    if now.elapsed() >= Duration::from_millis(5) {
                        break;
                    }
                }
            }
        }
    }

    alive_hosts
}
