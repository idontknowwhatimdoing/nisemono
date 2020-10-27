use crate::mac_addr::MacAddr;
use crate::frame::Frame;
use smoltcp::phy::{wait, Device, RawSocket, RxToken};
use smoltcp::time::Instant;
use std::os::unix::io::AsRawFd;

fn forward_packets(buffer: &mut [u8], target_mac: MacAddr, _socket: &mut RawSocket) -> Result<(), smoltcp::Error> {
    if Frame::from(&buffer[..14]).dst == target_mac {
        println!("recieved data from : {}", target_mac);
        Ok(())
    } else {
        Err(smoltcp::Error::__Nonexhaustive)
    }
}

pub fn listen(socket: &mut RawSocket, target_mac: MacAddr) {
    loop {
        wait(socket.as_raw_fd(), None).unwrap();
        let (rx, _) = socket.receive().unwrap();
        let _ = rx.consume(Instant::now(), |buffer| forward_packets(buffer, target_mac, socket));
    }
}
