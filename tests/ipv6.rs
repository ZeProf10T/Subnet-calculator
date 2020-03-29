use std::net::Ipv6Addr;

use subnet_calculator::ipv6;

#[cfg(test)]
mod address {
    use super::*;

    #[test]
    fn basic_cidr_to_mask() {
        assert_eq!(
            ipv6::utils::cidr_to_mask(64),
            Ipv6Addr::new(0xffff,0xffff,0xffff,0xffff,0,0,0,0)
        )
    }

    #[test]
    fn complex_cidr_to_mask() {
        assert_eq!(
            ipv6::utils::cidr_to_mask(66),
            Ipv6Addr::new(0xffff,0xffff,0xffff,0xffff,0xc000,0,0,0)
        )
    }



}