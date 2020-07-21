use smoltcp::phy::{wait, Device, RawSocket, RxToken};
use smoltcp::time::Instant;
use std::os::unix::io::AsRawFd;
use systemstat::{Platform, System};

fn get_iface_name() -> Option<String> {
	let sys = System::new();
	match sys.networks() {
		Ok(ifaces) => {
			for iface in ifaces.values() {
				if iface.name != "lo" {
					return Some(iface.name.clone());
				}
			}
		}
		Err(e) => println!("\nnetworks error: {}", e),
	}

	None
}

fn build_socket() -> Option<RawSocket> {
	match get_iface_name() {
		Some(iface) => {
			println!("\nusing network interface {}\n", iface);
			match RawSocket::new(iface.as_str()) {
				Ok(socket) => return Some(socket),
				Err(e) => println!("\nsocket creation error : {}", e),
			}
		}
		None => println!("\nno network interface found ..."),
	}

	None
}

fn is_arp(buffer: &mut [u8]) -> bool {
	buffer[12] == 8 && buffer[13] == 6
}

fn handle_frame(buffer: &mut [u8]) -> Result<(), smoltcp::Error> {
	if is_arp(buffer) {
		println!("{:x?}\n", buffer);
	}

	Ok(())
}

fn capture_frames(mut socket: RawSocket) {
	loop {
		wait(socket.as_raw_fd(), None).unwrap();
		let (rx, _) = socket.receive().unwrap();
		rx.consume(Instant::now(), |buffer| handle_frame(buffer))
			.unwrap();
	}
}

fn main() {
	match build_socket() {
		Some(socket) => capture_frames(socket),
		None => println!("could not create the socket ..."),
	}
}
