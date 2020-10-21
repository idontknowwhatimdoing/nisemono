use crate::mac_addr::MacAddr;
use crate::utils::get_local_mac;
use std::env::args;

pub type Word = [u8; 2];

#[derive(Clone, Copy)]
pub struct Frame {
    pub dst: MacAddr,
    pub src: MacAddr,
    pub ether_type: Word
}

impl Frame {
    pub fn new(dst: MacAddr, src: MacAddr, ether_type: Word) -> Self {
        Frame { dst, src, ether_type }
    }

    pub fn new_arp_request_frame() -> Self {
        Frame {
            dst: MacAddr::BROADCAST,
            src: get_local_mac(&args().nth(1).unwrap()),
            ether_type: [8, 6]
        }
    }
}
