use regex::Regex;

// Given a network address, returns all addresses that could be in use
pub fn _bruteforce_addrs(netaddr: String) -> Result<Vec<[u8; 4]>, String> {
	let re = Regex::new(r"(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})/(\d+)").unwrap();
	if re.is_match(netaddr.as_str()) {
		let caps = re.captures(netaddr.as_str()).unwrap();
		let cidr: u32 = caps.get(5).unwrap().as_str().parse().unwrap();

		let mut nb_hosts: u32 = 0;
		for i in 0..32 - cidr {
			nb_hosts += 1 * 2u32.pow(i);
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

// Given all possible host addresses, performs a ping scan to reveal which hosts are alive
pub fn _scan_network(_hosts: Vec<String>) {}