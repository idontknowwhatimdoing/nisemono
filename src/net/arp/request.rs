use crate::net::utils::*;
use ansi_term::Color;
use smoltcp::phy::{wait, Device, RawSocket, RxToken, TxToken};
use smoltcp::time::Instant;
use std::os::unix::io::AsRawFd;

fn craft_frame(buffer: &mut [u8], target_ip: &[u8]) -> Result<(), smoltcp::Error> {
    let sender_mac = get_local_mac(get_iface_name().unwrap());
    let sender_ip = get_local_ip().unwrap();

    // dest
    for byte in buffer.iter_mut().take(6) {
        *byte = 0xff;
    }

    // src
    for (j, i) in (6..12).enumerate() {
        buffer[i] = sender_mac[j];
    }

    // type
    buffer[12] = 8;
    buffer[13] = 6;

    // hardware type
    buffer[14] = 0;
    buffer[15] = 1;

    // protocol type
    buffer[16] = 8;
    buffer[17] = 0;

    // hardware size
    buffer[18] = 6;

    // protocol size
    buffer[19] = 4;

    // opcode
    buffer[20] = 0;
    buffer[21] = 1;

    // sender MAC address
    for (j, i) in (22..28).enumerate() {
        buffer[i] = sender_mac[j];
    }

    // sender IP address
    for (j, i) in (28..32).enumerate() {
        buffer[i] = sender_ip[j];
    }

    // target_ip MAC address
    for byte in buffer.iter_mut().take(32).skip(38) {
        *byte = 0;
    }

    // target_ip IP address
    for (j, i) in (38..42).enumerate() {
        buffer[i] = target_ip[j];
    }

    Ok(())
}

pub fn send(socket: &mut RawSocket, target_ip: &[u8]) {
    let tx = socket.transmit().unwrap();
    tx.consume(Instant::now(), 42, |buffer| craft_frame(buffer, target_ip))
        .unwrap();
}

pub fn is_target_reply(buffer: &mut [u8], target_ip: &[u8]) -> bool {
    let local_mac = get_local_mac(get_iface_name().unwrap());
    let mut ip_ok = true;
    let mut mac_ok = true;

    // dest == local_mac
    for (j, i) in (0..6).enumerate() {
        if buffer[i] != local_mac[j] {
            mac_ok = false;
            break;
        }
    }

    // target_ip == sender_ip
    for (j, i) in (28..32).enumerate() {
        if buffer[i] != target_ip[j] {
            ip_ok = false;
            break;
        }
    }

    buffer[12] == 8 && buffer[13] == 6 && buffer[20] == 0 && buffer[21] == 2 && ip_ok && mac_ok
}

fn extract_mac(buffer: &mut [u8], target_ip: &[u8]) -> Result<[u8; 6], smoltcp::Error> {
    if is_target_reply(buffer, target_ip) {
        // extract target's MAC address
        let mut mac = [0; 6];

        for (j, i) in (6..12).enumerate() {
            mac[j] = buffer[i];
        }

        return Ok(mac);
    }

    Err(smoltcp::Error::__Nonexhaustive)
}

pub fn get_target_mac(socket: &mut RawSocket, target_ip: &[u8]) -> [u8; 6] {
    loop {
        wait(socket.as_raw_fd(), None).unwrap();
        let (rx, _) = socket.receive().unwrap();
        match rx.consume(Instant::now(), |buffer| extract_mac(buffer, target_ip)) {
            Ok(mac) => return mac,
            Err(_) => print!("{}", Color::Green.paint(".")),
        }
    }
}
