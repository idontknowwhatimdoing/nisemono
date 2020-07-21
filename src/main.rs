use smoltcp::phy::RawSocket;
use systemstat::{Platform, System};

fn get_iface_name() -> Option<String> {
	let sys = System::new();
	match sys.networks() {
		Ok(ifaces) => {
			for iface in ifaces.values() {
				println!("found network interface : {}", iface.name);
				if iface.name != "lo" {
					return Some(iface.name.clone());
				}
			}
		}
		Err(e) => println!("\nNetworks error: {}", e),
	}

	None
}

fn build_socket() -> Option<RawSocket> {
	match get_iface_name() {
		Some(iface) => {
			println!("using network interface {}", iface);
			return Some(RawSocket::new(iface.as_str()).unwrap());
		}
		None => println!("no network interface found ..."),
	}

	None
}

fn main() {
	let mut sock = build_socket().unwrap();
}
