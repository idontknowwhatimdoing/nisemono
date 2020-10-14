type MacAddr = [u8; 6];
type Ipv4Addr = [u8; 4];
type Ipv6Addr = [u8; 6];

struct Frame {
    dest: MacAddr,
    src: MacAddr,
    ether_type: [u8; 2]
}

impl Frame {
    fn new(dest: MacAddr, src: MacAddr, ether_type: [u8; 2]) -> Frame {
        Frame {
            dest,
            src,
            ether_type
        }
    }
}

struct ArpFrame {
    frame: Frame,
    htype: [u8; 2],
    ptype: [u8; 2],
    hlen: u8,
    plen: u8,
    op: [u8; 2],
    sha: MacAddr,
    spa: Ipv4Addr,
    tha: MacAddr,
    tpa: Ipv4Addr
}
