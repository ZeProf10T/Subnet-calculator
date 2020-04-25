use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

#[derive(Debug, PartialEq)]
pub enum Version {
    Four,
    Six
}

pub fn version(address: &str) -> Result<Version,&'static str> {
    match address.parse::<Ipv4Addr>() {
        Ok(i) => {
            return Ok(Version::Four);
        },
        _ => {}
    }

    match address.parse::<Ipv6Addr>() {
        Ok(i) => {
            return Ok(Version::Six);
        },
        _ => {}
    }

    Err("Invalid ip address")
}