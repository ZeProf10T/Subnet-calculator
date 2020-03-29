extern crate clap;

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use clap::{Arg, App};

pub mod ipv4;
mod ipv6;


fn quatre(ip: Ipv4Addr, mask: Ipv4Addr, subnet_mask: Ipv4Addr) {

    let network = ipv4::address::network(&ip, &mask);
    let wildcard = ipv4::address::wildcard(&mask);
    let broadcast = ipv4::address::broadcast(&ip, &mask);
    let cidr = ipv4::utils::mask_to_cidr(&mask);
    let first = ipv4::address::first(&network);
    let last = ipv4::address::last(&broadcast);
    let hosts = ipv4::utils::host_count(&wildcard);

    println!();

    println!("{} /{}", ip, cidr);
    println!("\tNetwork : {:#?}", network);
    println!("\tBroadcast : {:#?}", broadcast);
    println!("\tMask : {:#?}", mask);
    println!("\tWilcard : {:#?}", wildcard);

    println!("\tAvailable  : {} - {}", first, last);
    println!("\tFree Hosts : {}", hosts);

    println!();

    let subnet_wildcard = ipv4::address::wildcard(&subnet_mask);
    let subnet_count = ipv4::subnet::count(&wildcard, &subnet_wildcard);

    println!("Subnet-mask : {}", subnet_mask);
    println!("Number of sub-network : {}", subnet_count);


    let subnetworks = ipv4::subnet::calculate(&network, &mask, &subnet_mask);

    for subnetwork in subnetworks {

        let cidr = ipv4::utils::mask_to_cidr(&subnet_mask);
        let broadcast = ipv4::address::broadcast(&subnetwork, &subnet_mask);

        let first = ipv4::address::first(&subnetwork);
        let last = ipv4::address::last(&broadcast);

        let hosts = ipv4::utils::host_count(&ipv4::address::wildcard(&subnet_mask));

        println!();
        println!("{} /{}", subnetwork, cidr);
        println!("\tAvailable  : {} - {}", first, last);
        println!("\tFree Hosts : {}", hosts);
        println!("\tBroadcast  : {}", broadcast);
    }


    println!();
}

fn six(ip: Ipv6Addr, mask: Ipv6Addr, subnet_mask: Ipv6Addr) {

    let network = ipv6::address::network(&ip, &mask);
    let wildcard = ipv6::address::wildcard(&mask);
    let broadcast = ipv6::address::broadcast(&ip, &wildcard);

    let first = ipv6::address::first(&network);
    let last = ipv6::address::last(&broadcast);

    let hosts = ipv6::utils::host_count(&wildcard);
    let cidr = ipv6::utils::mask_to_cidr(&mask);

    println!();

    println!("{} /{}", network, cidr);
    println!("\tMask : {:#?}", mask);
    println!("\tWilcard : {:#?}", wildcard);

    println!("\tAvailable  : {} - {}", first, last);
    println!("\tFree Hosts : {}", hosts);


    /* SUBNETWORK */
    let subnet_wildcard = ipv6::address::wildcard(&subnet_mask);
    let count = ipv6::subnet::count(&wildcard, &subnet_wildcard);

    println!("");
    println!("Subnet-mask : {}", subnet_mask);
    println!("Number of sub-network : {}", count);

    let subnetworks = ipv6::subnet::calculate(&network, &mask, &subnet_mask);


    for subnetwork in subnetworks {

        let cidr = ipv6::utils::mask_to_cidr(&subnet_mask);

        let first = ipv6::address::first(&subnetwork);
        let last = ipv6::address::last(&broadcast);

        let hosts = ipv6::utils::host_count(&ipv6::address::wildcard(&subnet_mask));

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
                                1..=32 => {
                                    let mask = ipv4::utils::cidr_to_mask(a);
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
                                1..=32 => {
                                    let subnet_mask = ipv4::utils::cidr_to_mask(a);
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

