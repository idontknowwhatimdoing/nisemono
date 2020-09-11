use smoltcp::phy::{wait, Device, RawSocket, RxToken};
use smoltcp::time::Instant;
use std::os::unix::io::AsRawFd;

fn _is_from_target(buffer: &mut [u8], target_a_mac: &[u8], target_b_mac: &[u8]) -> bool {
    let mut src = true;

    // check if the packet comes from one of the targets
    for (j, byte) in buffer.iter_mut().take(11).skip(6).enumerate() {
        if *byte != target_a_mac[j] {
            src = false;
            break;
        }
    }
    if !src {
        for (j, byte) in buffer.iter_mut().take(11).skip(6).enumerate() {
            src = *byte == target_b_mac[j];
            if !src {
                break;
            }
        }
    }

    src
}

fn _forward_packets(
    buffer: &mut [u8],
    target_a_mac: &[u8],
    target_b_mac: &[u8],
) -> Result<(), smoltcp::Error> {
    if _is_from_target(buffer, target_a_mac, target_b_mac) {
        // send packet to the actual dest
    }

    Ok(())
}

pub fn _listen(socket: &mut RawSocket, target_a_mac: &[u8], target_b_mac: &[u8]) {
    loop {
        wait(socket.as_raw_fd(), None).unwrap();
        let (rx, _) = socket.receive().unwrap();
        rx.consume(Instant::now(), |buffer| {
            _forward_packets(buffer, target_a_mac, target_b_mac)
        })
        .unwrap();
    }
}
