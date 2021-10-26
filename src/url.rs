use std::net::Ipv4Addr;

use anyhow::{Error, Result};
use base64::{decode, encode};
use getrandom::getrandom;

pub const TXRX_SCHEME: &str = "txrx://";

/// Url format
/// txrx://[4 bytes secret for handshake][[4 bytes IPv4 address]...]
/// It at least 7+4+4 bytes long
#[derive(Debug, Clone)]
pub struct Url {
    pub secret: [u8; 4],
    pub ips: Vec<Ipv4Addr>,
}

impl Url {
    pub fn new(ips: &[Ipv4Addr]) -> Self {
        let mut secret = [0u8; 4];
        getrandom(&mut secret).expect("Broken random generator, strange OS?");
        Self {
            secret,
            ips: ips.to_vec(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        if !s.starts_with(TXRX_SCHEME) {
            return Err(Error::msg("Wrong txrx Url scheme"));
        }
        let data: Vec<u8> = decode(s[TXRX_SCHEME.len()..].as_bytes())?;

        if data.len() % 4 != 0 || data.len() < 8 {
            return Err(Error::msg("Wrong txrx Url length"));
        }

        let mut r = Url {
            secret: [0u8; 4],
            ips: Vec::with_capacity((data.len() / 4) - 1),
        };

        let mut iter = data.chunks_exact(4);
        // 1. Read secret
        // Had make sure it has 4 bytes
        r.secret.copy_from_slice(iter.next().unwrap());
        // 2. Read ips
        iter.for_each(|x| r.ips.push(Ipv4Addr::new(x[0], x[1], x[2], x[3])));

        Ok(r)
    }
}

impl ToString for Url {
    fn to_string(&self) -> String {
        let mut data: Vec<u8> = Vec::with_capacity(4 + self.ips.len() * 4);
        data.extend_from_slice(&self.secret);
        self.ips.iter().for_each(|x| data.extend_from_slice(&x.octets()));
        return format!("{}{}", TXRX_SCHEME, encode(data));
    }
}
