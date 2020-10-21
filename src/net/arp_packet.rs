use std::net::Ipv4Addr;
use smoltcp::time::Instant;
use smoltcp::phy::{Device, RawSocket, TxToken};
use crate::mac_addr::MacAddr;
use crate::frame::{Frame, Word};
use crate::utils;

pub struct ArpPacket {
    pub frame: Frame,
    pub htype: Word,
    pub ptype: Word,
    pub hlen: u8,
    pub plen: u8,
    pub oper: Word,
    pub sha: MacAddr,
    pub spa: Ipv4Addr,
    pub tha: MacAddr,
    pub tpa: Ipv4Addr
}

impl ArpPacket {
    pub fn new_request(frame: Frame, sender_ip: Ipv4Addr, target_ip: Ipv4Addr) -> Self {
        ArpPacket {
            frame,
            htype: [0, 1],
            ptype: [8, 0],
            hlen: 6,
            plen: 4,
            oper: [0, 1],
            sha: frame.src,
            spa: sender_ip,
            tha: MacAddr::UNSPECIFIED,
            tpa: target_ip
        }
    }

    pub fn new_reply(frame: Frame, target_mac: MacAddr, target_ip: Ipv4Addr, sender_ip: Ipv4Addr) -> Self {
        ArpPacket {
            frame,
            htype: [0, 1],
            ptype: [8, 0],
            hlen: 6,
            plen: 4,
            oper: [0, 2],
            sha: frame.src,
            spa: sender_ip,
            tha: target_mac,
            tpa: target_ip
        }
    }

    pub fn is_reply(&self) -> bool {
        self.oper == [0, 2]
    }

    pub fn fill_buffer(&self, buffer: &mut [u8]) {
        let mut buffer = Vec::from(buffer);

        buffer.extend_from_slice(&self.frame.dst.octets());
        buffer.extend_from_slice(&self.frame.src.octets());
        buffer.extend_from_slice(&self.frame.ether_type);
        buffer.extend_from_slice(&self.htype);
        buffer.extend_from_slice(&self.ptype);
        buffer.push(self.hlen);
        buffer.push(self.plen);
        buffer.extend_from_slice(&self.oper);
        buffer.extend_from_slice(&self.sha.octets());
        buffer.extend_from_slice(&self.spa.octets());
        buffer.extend_from_slice(&self.tha.octets());
        buffer.extend_from_slice(&self.tpa.octets());
    }
}

pub fn send_request(socket: &mut RawSocket, target_ip: &Ipv4Addr) {
    let req = ArpPacket::new_request(
        Frame::new_arp_request_frame(),
        utils::get_local_ip().unwrap(),
        *target_ip        
    );

    let tx = socket.transmit().unwrap();
    tx.consume(Instant::now(), 42, |buffer| {
        Ok(())
    }).unwrap();
}
