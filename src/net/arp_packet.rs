use std::net::Ipv4Addr;
use std::env::args;
use std::os::unix::io::AsRawFd;
use std::convert::TryFrom;
use smoltcp::time::Instant;
use smoltcp::phy::{wait, Device, RawSocket, RxToken, TxToken};
use crate::mac_addr::MacAddr;
use crate::frame::{Frame, Word};
use crate::utils::{get_local_ip, get_local_mac};

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
    fn new_request(frame: Frame, sender_ip: Ipv4Addr, target_ip: Ipv4Addr) -> Self {
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

    fn new_reply(frame: Frame, target_ip: Ipv4Addr, sender_ip: Ipv4Addr) -> Self {
        ArpPacket {
            frame,
            htype: [0, 1],
            ptype: [8, 0],
            hlen: 6,
            plen: 4,
            oper: [0, 2],
            sha: frame.src,
            spa: sender_ip,
            tha: frame.dst,
            tpa: target_ip
        }
    }

    fn is_reply(&self) -> bool {
        self.oper == [0, 2]
    }

    fn is_dst_local(&self) -> bool {
        self.tha == get_local_mac(&args().nth(1).unwrap())
    }

    fn fill_buffer(&self, buffer: &mut [u8]) {
        let mut i = 0;

        for byte in self.frame.dst.octets().iter() {
            buffer[i] = *byte;
            i += 1;
        }
        for byte in self.frame.src.octets().iter() {
            buffer[i] = *byte;
            i += 1;
        }
        for byte in self.frame.ether_type.iter() {
            buffer[i] = *byte;
            i += 1;
        }
        for byte in self.htype.iter() {
            buffer[i] = *byte;
            i += 1;
        }
        for byte in self.ptype.iter() {
            buffer[i] = *byte;
            i += 1;
        }
        buffer[i] = self.hlen;
        i +=1;
        buffer[i] = self.plen;
        i += 1;
        for byte in self.oper.iter() {
            buffer[i] = *byte;
            i += 1;
        }
        for byte in self.sha.octets().iter() {
            buffer[i] = *byte;
            i += 1;
        }
        for byte in self.spa.octets().iter() {
            buffer[i] = *byte;
            i += 1;
        }
        for byte in self.tha.octets().iter() {
            buffer[i] = *byte;
            i += 1;
        }
        for byte in self.tpa.octets().iter() {
            buffer[i] = *byte;
            i += 1;
        }
    }

    pub fn send_request(socket: &mut RawSocket, target_ip: &Ipv4Addr) {
        let req = ArpPacket::new_request(
            Frame::new(MacAddr::BROADCAST, get_local_mac(&args().nth(1).unwrap()), [8, 6]),
            get_local_ip().unwrap(),
            *target_ip        
        );

        let tx = socket.transmit().unwrap();
        tx.consume(Instant::now(), 42, |buffer| {
            req.fill_buffer(buffer);
            Ok(())
        }).unwrap();
    }

    pub fn send_reply(socket: &mut RawSocket, target_mac: &MacAddr, target_ip: &Ipv4Addr, sender_ip: &Ipv4Addr) {
        let rep = ArpPacket::new_reply(
            Frame::new(*target_mac, get_local_mac(&args().nth(1).unwrap()), [8, 6]),
            *target_ip,
            *sender_ip
        );

        let tx = socket.transmit().unwrap();
        tx.consume(Instant::now(), 42, |buffer| {
            rep.fill_buffer(buffer);
            Ok(())
        }).unwrap();
    }
}

impl TryFrom<&mut [u8]> for ArpPacket {
    type Error = ();

    fn try_from(buffer: &mut [u8]) -> Result<Self, Self::Error> {
        if [buffer[12], buffer[13]] == [8, 6] {
            Ok(ArpPacket {
                frame: Frame::from(&buffer[..14]),
                htype: [buffer[14], buffer[15]],
                ptype: [buffer[16], buffer[17]],
                hlen: buffer[18],
                plen: buffer[19],
                oper: [buffer[20], buffer[21]],
                sha: MacAddr::from(&buffer[22..28]),
                spa: Ipv4Addr::new(buffer[28], buffer[29], buffer[30], buffer[31]),
                tha: MacAddr::from(&buffer[32..38]),
                tpa: Ipv4Addr::new(buffer[38], buffer[39], buffer[40], buffer[41])
            })
        } else {
            Err(())
        }
    }
}

pub fn get_target_mac(socket: &mut RawSocket, target_ip: &Ipv4Addr) -> MacAddr {
    loop {
        wait(socket.as_raw_fd(), None).unwrap();
        let (rx, _) = socket.receive().unwrap();
        if let Ok(mac) = rx.consume(Instant::now(), |buffer| {
            if let Ok(packet) = ArpPacket::try_from(buffer) {
                if packet.is_reply() && packet.is_dst_local() && packet.spa == *target_ip {
                    return Ok(packet.sha);
                }
            }
            Err(smoltcp::Error::__Nonexhaustive)
        }) { return mac; }
    }
}
