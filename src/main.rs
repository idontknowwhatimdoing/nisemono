mod net;
use net::*;
use std::env::args;

fn main() {
	if args().len() == 3 {
		match utils::build_socket() {
			Ok(mut socket) => arp::send_request(&mut socket),
			Err(e) => eprintln!("{}", e),
		}
	} else {
		println!("usage : sudo ./arp-spoof <target 1 IP> <target 2 IP>");
	}
}
