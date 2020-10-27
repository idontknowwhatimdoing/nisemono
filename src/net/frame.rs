use std::convert::From;
use crate::mac_addr::MacAddr;

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
}

impl From<&[u8]> for Frame {
    fn from(buffer: &[u8]) -> Self {
        Frame::new(MacAddr::from(&buffer[..6]), MacAddr::from(&buffer[6..12]), [buffer[12], buffer[13]])
    }
}
