use crate::mac_addr::MacAddr;
use crate::utils::get_local_mac;
use smoltcp::phy::{wait, Device, RawSocket, RxToken, TxToken};
use smoltcp::time::Instant;
use std::os::unix::io::AsRawFd;
//use std::{thread, time};
use std::env::args;

fn redirect(socket: &mut RawSocket, buffer: &mut [u8], target_mac: MacAddr) {
    println!("recieved:\n\tdst: {}\n\tsrc: {}", MacAddr::from(&buffer[..6]), MacAddr::from(&buffer[6..12]));
    let tx = socket.transmit().unwrap();
    tx.consume(Instant::now(), buffer.len(), |new| {
        new.copy_from_slice(buffer);
        for (i, byte) in target_mac.octets().iter().enumerate() { new[i] = *byte; }
        println!("sending:\n\tdst: {}\n\tsrc: {}", MacAddr::from(&new[..6]), MacAddr::from(&new[6..12]));
        Ok(())
    }).unwrap();
    //thread::sleep(time::Duration::from_micros(300));
}

fn is_from_target(buffer: &mut [u8], src: MacAddr) -> bool {
    MacAddr::from(&buffer[6..12]) == src && MacAddr::from(&buffer[..6]) == get_local_mac(&args().nth(1).unwrap())
}

pub fn listen(socket: &mut RawSocket, target_a_mac: MacAddr, target_b_mac: MacAddr) {
    loop {
        wait(socket.as_raw_fd(), None).unwrap();
        let (rx, _) = socket.receive().unwrap();
        rx.consume(Instant::now(), |buffer| {
            if [buffer[12], buffer[13]] == [8, 0] && buffer[23] == 1 {
                if is_from_target(buffer, target_a_mac) { redirect(socket, buffer, target_b_mac); }
                else if is_from_target(buffer, target_b_mac) { redirect(socket, buffer, target_a_mac); }
            }
            Ok(())
        }).unwrap();
    }
}
