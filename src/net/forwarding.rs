use smoltcp::phy::{wait, Device, RawSocket, RxToken};
use smoltcp::time::Instant;
use std::os::unix::io::AsRawFd;

fn _is_from_target<'a>(buffer: &'a mut [u8], target_a_mac: &'a[u8], target_b_mac: &'a[u8]) -> Option<&'a[u8]> {
    for (j, byte) in buffer.iter_mut().take(11).skip(6).enumerate() {
        if *byte != target_a_mac[j] {
            for (j, byte) in buffer.iter_mut().take(11).skip(6).enumerate() {
                if *byte != target_b_mac[j] {
                    return None;
                }
            }

            return Some(target_b_mac);
        }
    }

    Some(target_a_mac)
}

fn _forward_packets(buffer: &mut [u8], target_a_mac: &[u8], target_b_mac: &[u8], _socket: &mut RawSocket) -> Result<(), smoltcp::Error> {
    match _is_from_target(buffer, target_a_mac, target_b_mac) {
        Some(sender) => {
            println!("{:?}", sender);
            return Ok(());
        },
        None => Err(smoltcp::Error::__Nonexhaustive)
    }
}

pub fn _listen(socket: &mut RawSocket, target_a_mac: &[u8], target_b_mac: &[u8]) {
    loop {
        wait(socket.as_raw_fd(), None).unwrap();
        let (rx, _) = socket.receive().unwrap();
        rx.consume(Instant::now(), |buffer| _forward_packets(buffer, target_a_mac, target_b_mac, socket)).unwrap();
    }
}
