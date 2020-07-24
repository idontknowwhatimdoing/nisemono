mod net;

fn main() {
	match net::build_socket() {
		Ok(mut socket) => net::send_arp_request(&mut socket),
		Err(e) => println!("{}", e),
	}
}
