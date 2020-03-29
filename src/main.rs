extern crate clap;

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use clap::{Arg, App};
use crate::IPv4::utils::{mask_to_cidr, cidr_to_mask};

mod IPv4;
mod IPv6;


fn quatre(ip: Ipv4Addr, mask: Ipv4Addr, subnet_mask: Ipv4Addr) {

    let network = IPv4::address::network(&ip, &mask);
    let wildcard = IPv4::address::wildcard(&mask);
    let broadcast = IPv4::address::broadcast(&ip, &mask);
    let cidr = IPv4::utils::mask_to_cidr(&mask);
    let first = IPv4::address::first(&network);
    let last = IPv4::address::last(&broadcast);
    let hosts = IPv4::utils::host_count(&wildcard);

    println!();

    println!("{} /{}", ip, cidr);
    println!("\tNetwork : {:#?}", network);
    println!("\tBroadcast : {:#?}", broadcast);
    println!("\tMask : {:#?}", mask);
    println!("\tWilcard : {:#?}", wildcard);

    println!("\tAvailable  : {} - {}", first, last);
    println!("\tFree Hosts : {}", hosts);

    println!();

    let subnet_wildcard = IPv4::address::wildcard(&subnet_mask);
    let subnet_count = IPv4::subnet::count(&wildcard, &subnet_wildcard);

    println!("Subnet-mask : {}", subnet_mask);
    println!("Number of sub-network : {}", subnet_count);


    let subnetworks = IPv4::subnet::calculate(&network,&mask,&subnet_mask);

    for subnetwork in subnetworks {

        let cidr = IPv4::utils::mask_to_cidr(&subnet_mask);
        let broadcast = IPv4::address::broadcast(&subnetwork,&subnet_mask);

        let first = IPv4::address::first(&subnetwork);
        let last = IPv4::address::last(&broadcast);

        let hosts = IPv4::utils::host_count(&IPv4::address::wildcard(&subnet_mask));

        println!();
        println!("{} /{}", subnetwork, cidr);
        println!("\tAvailable  : {} - {}", first, last);
        println!("\tFree Hosts : {}", hosts);
        println!("\tBroadcast  : {}", broadcast);
    }


    println!();
}

fn six(ip: Ipv6Addr, mask: Ipv6Addr, subnet_mask: Ipv6Addr) {

    let network = IPv6::address::network(&ip,&mask);
    let wildcard = IPv6::address::wildcard(&mask);
    let broadcast = IPv6::address::broadcast(&ip, &wildcard);

    let first = IPv6::address::first(&network);
    let last = IPv6::address::last(&broadcast);

    let hosts = IPv6::utils::host_count(&wildcard);
    let cidr = IPv6::utils::mask_to_cidr(&mask);

    println!();

    println!("{} /{}", network, cidr);
    println!("\tMask : {:#?}", mask);
    println!("\tWilcard : {:#?}", wildcard);

    println!("\tAvailable  : {} - {}", first, last);
    println!("\tFree Hosts : {}", hosts);


    /* SUBNETWORK */
    let subnet_wildcard = IPv6::address::wildcard(&subnet_mask);
    let count = IPv6::subnet::count(&wildcard, &subnet_wildcard);

    println!("");
    println!("Subnet-mask : {}", subnet_mask);
    println!("Number of sub-network : {}", count);

    let subnetworks = IPv6::subnet::calculate(&network,&mask,&subnet_mask);


    for subnetwork in subnetworks {

        let cidr = IPv6::utils::mask_to_cidr(&subnet_mask);

        let first = IPv6::address::first(&subnetwork);
        let last = IPv6::address::last(&broadcast);

        let hosts = IPv6::utils::host_count(&IPv6::address::wildcard(&subnet_mask));

        println!();
        println!("{} /{}", subnetwork, cidr);
        println!("\tAvailable  : {} - {}", first, last);
        println!("\tFree Hosts : {}", hosts);
        println!("\tBroadcast  : {}", broadcast);
    }


    println!();
}



fn main() {


    let matches = App::new("Subnet calculator")
        .version("0.1.0")
        .author("Léo Huteau <huteau890@gmail.com>")
        .about("A basic subnet calculator")
        .arg(Arg::with_name("Version")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("Choose between 4 and 6")
        )
        .arg(Arg::with_name("IP Address")
            .required(true)
            .takes_value(true)
            .index(2)
            .help("Must be IPv4 or IPv6")
        ).arg(Arg::with_name("Network mask")
            .required(true)
            .takes_value(true)
            .index(3)
            .help("Mask must be like 255.255.255.0 for IPv4 or  like ffff:ffff:ffff:ffff:: for IPv6")
        )
        .arg(Arg::with_name("Subnet mask")
            .required(true)
            .takes_value(true)
            .index(4)
            .help("Subnet-Mask must be like 255.255.255.0")
        )
        .get_matches();



    match matches.value_of("Version") {
        Some("4") => {
            /* Default value */
            let mut ip = Ipv4Addr::new(192,168,0,1);
            let mut mask = Ipv4Addr::new(255,255,255,0);
            let mut subnet_mask = Ipv4Addr::new(255,255,255,192);

            match matches.value_of("IP Address") {
                Some(a) => {
                    match a.parse::<Ipv4Addr>() {
                        Ok(b) => { ip = b; },
                        Err(_) => panic!("Non-valid IPv4")
                    }
                },
                None => panic!("Error")
            }

            match matches.value_of("Network mask") {
                Some(a) => {
                    match a.parse::<u8>() {
                        Ok(a) => {
                            match a {
                                1...31 => {
                                    let mask = cidr_to_mask(a);
                                },
                                _ => println!("Out of range")
                            }

                        }
                        _ => {
                            match a.parse::<Ipv4Addr>() {
                                Ok(b) => { mask = b; },
                                Err(_) => panic!("Non-valid subnet mask")
                            }
                        }

                    }

                },
                None => panic!("Error")
            }

            match matches.value_of("Subnet mask") {
                Some(a) => {
                    match a.parse::<u8>() {
                        Ok(a) => {
                            match a {
                                1...31 => {
                                    let subnet_cdmask = cidr_to_mask(a);
                                },
                                _ => println!("Out of range")
                            }

                        }
                        _ => {
                            match a.parse::<Ipv4Addr>() {
                                Ok(b) => { subnet_mask = b; },
                                Err(_) => panic!("Non-valid subnet mask")
                            }
                        }

                    }

                },
                None => panic!("Error")
            }

            quatre(ip, mask, subnet_mask);
        },
        Some("6") => {
            /* Default value */
            let mut ip = Ipv6Addr::new(0xfe80,0,0,1,0,0,0,1);
            let mut mask = Ipv6Addr::new(0xffff,0xffff,0xffff,0,0,0,0,0);
            let mut subnet_mask = Ipv6Addr::new(0xffff,0xffff,0xffff,0xffff,0,0,0,0);

            match matches.value_of("IP Address") {
                Some(a) => {
                    match a.parse::<Ipv6Addr>() {
                        Ok(b) => { ip = b; },
                        Err(_) => panic!("Non-valid IPv4")
                    }
                },
                None => panic!("Error")
            }

            match matches.value_of("Network mask") {
                Some(a) => {
                    match a.parse::<Ipv6Addr>() {
                        Ok(b) => { mask = b; },
                        Err(_) => panic!("Non-valid network mask")
                    }
                },
                None => panic!("Error")
            }

            match matches.value_of("Subnet mask") {
                Some(a) => {
                    match a.parse::<Ipv6Addr>() {
                        Ok(b) => { subnet_mask = b; },
                        Err(_) => panic!("Non-valid subnet mask")
                    }
                },
                None => panic!("Error")
            }

            six(ip, mask, subnet_mask);
        },
        _ => eprintln!("You must choose an IP version : 4 or 6")
    }





}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ipv4_network_address() {
        assert_eq!(
            IPv4::address::network(&Ipv4Addr::new(192, 168, 5, 35), &Ipv4Addr::new(255, 255, 255, 0)),
            Ipv4Addr::new(192, 168, 5, 0)
        );
    }

    #[test]
    fn complex_ipv4_network_address() {
        assert_eq!(
            IPv4::address::network(&Ipv4Addr::new(10,0,35,26), &Ipv4Addr::new(255,255,224,0)),
            Ipv4Addr::new(10,0,32,0)
        )
    }

    #[test]
    fn basic_ipv4_wildcard_address() {
        assert_eq!(
            IPv4::address::wildcard(&Ipv4Addr::new(255,255,255,0)),
            Ipv4Addr::new(0,0,0,255)
        )
    }

    #[test]
    fn complex_ipv4_wildcard_address() {
        assert_eq!(
            IPv4::address::wildcard(&Ipv4Addr::new(255,255,192,0)),
            Ipv4Addr::new(0,0,63,255)
        )
    }

    #[test]
    fn basic_ipv4_broadcast_address() {
        assert_eq!(
            IPv4::address::broadcast(&Ipv4Addr::new(192,168,0,5), &Ipv4Addr::new(255,255,255,0)),
            Ipv4Addr::new(192,168,0,255)
        )
    }

    #[test]
    fn complex_ipv4_broadcast_address() {
        assert_eq!(
            IPv4::address::broadcast(&Ipv4Addr::new(10,0,35,27), &Ipv4Addr::new(255,255,192,0)),
            Ipv4Addr::new(10,0,63,255)
        )
    }

    #[test]
    fn basic_ipv4_first_address() {
        assert_eq!(
            IPv4::address::first(&Ipv4Addr::new(192,168,0,0)),
            Ipv4Addr::new(192,168,0,1)
        )
    }

    #[test]
    fn basic_ipv4_last_address() {
        assert_eq!(
            IPv4::address::last(&Ipv4Addr::new(192,168,0,255)),
            Ipv4Addr::new(192,168,0,254)
        )
    }

    #[test]
    fn basic_ipv4_subnet_count() {
        assert_eq!(
            IPv4::subnet::count(&Ipv4Addr::new(0,0,1,255), &Ipv4Addr::new(0,0,0,255)),
            2
        )
    }

    #[test]
    fn complex_ipv4_subnet_count() {
        assert_eq!(
            IPv4::subnet::count(&Ipv4Addr::new(0,0,15,255), &Ipv4Addr::new(0,0,0,255)),
            16
        )


    }

    #[test]
    fn basic_ipv4_mask_to_cidr() {
        assert_eq!(
            IPv4::utils::mask_to_cidr(&Ipv4Addr::new(255,255,255,0)),
            24
        )
    }

    #[test]
    fn complex_ipv4_mask_to_cidr() {
        assert_eq!(
            IPv4::utils::mask_to_cidr(&Ipv4Addr::new(255,255,252,0)),
            22
        )
    }

    #[test]
    fn basic_ipv4_cidr_to_mask() {
        assert_eq!(
            IPv4::utils::cidr_to_mask(16),
            Ipv4Addr::new(255,255,0,0)
        )
    }

    #[test]
    fn complex_ipv4_cidr_to_mask() {
        assert_eq!(
            IPv4::utils::cidr_to_mask(18),
            Ipv4Addr::new(255,255,192,0)
        )
    }

    #[test]
    fn basic_ipv4_vec_to_num() {
        assert_eq!(
            IPv4::utils::vec_to_num([255,255,0,0]),
            0b11111111_11111111_00000000_00000000
        )
    }

    #[test]
    fn complex_ipv4_vec_to_num() {
        assert_eq!(
            IPv4::utils::vec_to_num([255,192,0,0]),
            0b11111111_11000000_00000000_00000000
        )
    }

    #[test]
    fn basic_ipv4_num_to_vec() {
        assert_eq!(
            IPv4::utils::num_to_vec(0b11111111_11111111_00000000_00000000),
            [255,255,0,0,]
        )
    }

    #[test]
    fn complex_ipv4_num_to_vec() {
        assert_eq!(
            IPv4::utils::num_to_vec(0b11111111_11000000_00000000_00000000),
            [255,192,0,0,]
        )
    }

    #[test]
    fn basic_ipv4_host_count() {
        assert_eq!(
            IPv4::utils::host_count(&Ipv4Addr::new(0,0,0,255)),
            254
        )
    }

    #[test]
    fn complex_ipv4_host_count() {
        assert_eq!(
            IPv4::utils::host_count(&Ipv4Addr::new(0,0,7,255)),
            2046
        )
    }
}