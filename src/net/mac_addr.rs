use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::convert::From;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct MacAddr {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8
}

impl MacAddr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        MacAddr { a, b, c, d, e, f }
    }

    pub fn octets(&self) -> [u8; 6] {
        [ self.a, self.b, self.c, self.d, self.e, self.f ]
    }

    pub const BROADCAST: Self = MacAddr::new(255, 255, 255, 255, 255, 255);
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}", self.a, self.b, self.c, self.d, self.e, self.f)
    }
}

impl FromStr for MacAddr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octets = [ 0, 0, 0, 0, 0, 0 ];

        for (i, byte) in s.trim().split(':').enumerate() {
            octets[i] = u8::from_str_radix(byte, 16)?;
        }

        Ok(MacAddr::from(octets))
    }
}

impl From<&[u8]> for MacAddr {
    fn from(octets: &[u8]) -> Self {
        MacAddr {
            a: octets[0],
            b: octets[1],
            c: octets[2],
            d: octets[3],
            e: octets[4],
            f: octets[5]
        }
    }
}

impl From<[u8; 6]> for MacAddr {
    fn from(octets: [u8; 6]) -> Self {
        MacAddr {
            a: octets[0],
            b: octets[1],
            c: octets[2],
            d: octets[3],
            e: octets[4],
            f: octets[5]
        }
    }
}
