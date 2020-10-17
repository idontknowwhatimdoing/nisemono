use crate::net::utils::*;
use crate::net::mac_addr::MacAddr;
use smoltcp::phy::{wait, Device, RawSocket, RxToken, TxToken};
use smoltcp::time::Instant;
use std::os::unix::io::AsRawFd;
use std::env::args;
use std::net::Ipv4Addr;

fn craft_frame(buffer: &mut [u8], target_ip: &[u8]) -> Result<(), smoltcp::Error> {
    let sender_mac = get_local_mac(&args().nth(1).unwrap()).octets();
    let sender_ip = get_local_ip().unwrap().octets();

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
    tx.consume(Instant::now(), 42, |buffer| craft_frame(buffer, target_ip)).unwrap();
}

pub fn is_target_reply(buffer: &mut [u8], target_ip: &Ipv4Addr) -> bool {
    let local_mac = get_local_mac(&args().nth(1).unwrap());
    let mac_ok = local_mac == MacAddr::from(&buffer[0..6]);
    let ip_ok = *target_ip == Ipv4Addr::new(buffer[28], buffer[29], buffer[30], buffer[31]);

    buffer[12] == 8 && buffer[13] == 6 && buffer[20] == 0 && buffer[21] == 2 && ip_ok && mac_ok
}

fn extract_mac(buffer: &mut [u8], target_ip: &Ipv4Addr) -> Result<MacAddr, smoltcp::Error> {
    if is_target_reply(buffer, target_ip) {
        return Ok(MacAddr::from(&buffer[6..12]));
    }

    Err(smoltcp::Error::__Nonexhaustive)
}

pub fn get_target_mac(socket: &mut RawSocket, target_ip: &Ipv4Addr) -> MacAddr {
    loop {
        wait(socket.as_raw_fd(), None).unwrap();
        let (rx, _) = socket.receive().unwrap();
        if let Ok(mac) = rx.consume(Instant::now(), |buffer| extract_mac(buffer, target_ip)) {
            return mac;
        }
    }
}
