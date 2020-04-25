use std::net::{Ipv4Addr, Ipv6Addr};

use subnet_calculator::ipv4;


#[cfg(test)]
mod address {
    use super::*;


    #[test]
    fn is_ipv4() {
        assert_eq!(
            version("192.168.0.20"),
            Ok(Version::Four)
        );

        assert_eq!(
            version("0.0.0.0"),
            Ok(Version::Four)
        );
    }

    #[test]
    fn is_ipv6() {
        assert_eq!(
            version("fe80::1"),
            Ok(Version::Six)
        );

        assert_eq!(
            version("2001:0:5:3::4e7b"),
            Ok(Version::Six)
        );
    }

    #[test]
    fn invalid_address() {
        assert_eq!(
            version("20001::1"),
            Err("Invalid ip address")
        );

        assert_eq!(
            version("256.28.17.3"),
            Err("Invalid ip address")
        );
    }


}