use std::net::Ipv4Addr;

use subnet_calculator::ipv4;

#[cfg(test)]
mod address {
    use super::*;


    #[test]
    fn basic_network() {
        assert_eq!(
            ipv4::address::network(&Ipv4Addr::new(192, 168, 5, 35), &Ipv4Addr::new(255, 255, 255, 0)),
            Ipv4Addr::new(192, 168, 5, 0)
        );
    }

    #[test]
    fn complex_network() {
        assert_eq!(
            ipv4::address::network(&Ipv4Addr::new(10, 0, 35, 26), &Ipv4Addr::new(255, 255, 224, 0)),
            Ipv4Addr::new(10,0,32,0)
        )
    }

    #[test]
    fn basic_wildcard() {
        assert_eq!(
            ipv4::address::wildcard(&Ipv4Addr::new(255, 255, 255, 0)),
            Ipv4Addr::new(0,0,0,255)
        )
    }

    #[test]
    fn complex_wildcard() {
        assert_eq!(
            ipv4::address::wildcard(&Ipv4Addr::new(255, 255, 192, 0)),
            Ipv4Addr::new(0,0,63,255)
        )
    }

    #[test]
    fn basic_broadcast() {
        assert_eq!(
            ipv4::address::broadcast(&Ipv4Addr::new(192, 168, 0, 5), &Ipv4Addr::new(255, 255, 255, 0)),
            Ipv4Addr::new(192,168,0,255)
        )
    }

    #[test]
    fn complex_broadcast() {
        assert_eq!(
            ipv4::address::broadcast(&Ipv4Addr::new(10, 0, 35, 27), &Ipv4Addr::new(255, 255, 192, 0)),
            Ipv4Addr::new(10,0,63,255)
        )
    }

    #[test]
    fn basic_first() {
        assert_eq!(
            ipv4::address::first(&Ipv4Addr::new(192, 168, 0, 0)),
            Ipv4Addr::new(192,168,0,1)
        )
    }

    #[test]
    fn basic_last() {
        assert_eq!(
            ipv4::address::last(&Ipv4Addr::new(192, 168, 0, 255)),
            Ipv4Addr::new(192,168,0,254)
        )
    }


}

#[cfg(test)]
mod utils {
    use super::*;

    #[test]
    fn basic_subnet_count() {
        assert_eq!(
            ipv4::subnet::count(&Ipv4Addr::new(0, 0, 1, 255), &Ipv4Addr::new(0, 0, 0, 255)),
            2
        )
    }

    #[test]
    fn complex_subnet_count() {
        assert_eq!(
            ipv4::subnet::count(&Ipv4Addr::new(0, 0, 15, 255), &Ipv4Addr::new(0, 0, 0, 255)),
            16
        )


    }

    #[test]
    fn basic_mask_to_cidr() {
        assert_eq!(
            ipv4::utils::mask_to_cidr(&Ipv4Addr::new(255, 255, 255, 0)),
            24
        )
    }

    #[test]
    fn complex_mask_to_cidr() {
        assert_eq!(
            ipv4::utils::mask_to_cidr(&Ipv4Addr::new(255, 255, 252, 0)),
            22
        )
    }

    #[test]
    fn basic_cidr_to_mask() {
        assert_eq!(
            ipv4::utils::cidr_to_mask(16),
            Ipv4Addr::new(255,255,0,0)
        )
    }

    #[test]
    fn complex_cidr_to_mask() {
        assert_eq!(
            ipv4::utils::cidr_to_mask(18),
            Ipv4Addr::new(255,255,192,0)
        )
    }

    #[test]
    fn basic_vec_to_num() {
        assert_eq!(
            ipv4::utils::vec_to_num([255,255,0,0]),
            0b11111111_11111111_00000000_00000000
        )
    }

    #[test]
    fn complex_vec_to_num() {
        assert_eq!(
            ipv4::utils::vec_to_num([255,192,0,0]),
            0b11111111_11000000_00000000_00000000
        )
    }

    #[test]
    fn basic_num_to_vec() {
        assert_eq!(
            ipv4::utils::num_to_vec(0b11111111_11111111_00000000_00000000),
            [255,255,0,0,]
        )
    }

    #[test]
    fn complex_num_to_vec() {
        assert_eq!(
            ipv4::utils::num_to_vec(0b11111111_11000000_00000000_00000000),
            [255,192,0,0,]
        )
    }

    #[test]
    fn basic_host_count() {
        assert_eq!(
            ipv4::utils::host_count(&Ipv4Addr::new(0, 0, 0, 255)),
            254
        )
    }

    #[test]
    fn complex_host_count() {
        assert_eq!(
            ipv4::utils::host_count(&Ipv4Addr::new(0, 0, 7, 255)),
            2046
        )
    }
}