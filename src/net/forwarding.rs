use smoltcp::phy::{wait, Device, RawSocket, RxToken};
use smoltcp::time::Instant;
use std::os::unix::io::AsRawFd;

fn is_from_target<'a, 'b>(buffer: &'b mut [u8], target_a_mac: &'a[u8], target_b_mac: &'a[u8]) -> Option<&'a[u8]> {
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

fn forward_packets(buffer: &mut [u8], target_a_mac: &[u8], target_b_mac: &[u8], _socket: &mut RawSocket) -> Result<(), smoltcp::Error> {
    match is_from_target(buffer, target_a_mac, target_b_mac) {
        Some(sender) => {
            println!("recieved data from : {:?}", sender);
            Ok(())
        },
        None => Err(smoltcp::Error::__Nonexhaustive)
    }
}

pub fn listen(socket: &mut RawSocket, target_a_mac: &[u8], target_b_mac: &[u8]) {
    loop {
        wait(socket.as_raw_fd(), None).unwrap();
        let (rx, _) = socket.receive().unwrap();
        let _ = rx.consume(Instant::now(), |buffer| forward_packets(buffer, target_a_mac, target_b_mac, socket));
    }
}
