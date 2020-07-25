mod net;
use net::*;

fn main() {
	match utils::build_socket() {
		Ok(mut socket) => arp::send_request(&mut socket),
		Err(e) => println!("{}", e),
	}
}
