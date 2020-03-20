extern crate clap;

use std::net::Ipv4Addr;
use clap::{Arg, App};

mod IPv4;


fn main() {
    let matches = App::new("Subnet calculator")
        .version("0.1.0")
        .author("Léo Huteau <huteau890@gmail.com>")
        .about("A basic subnet calculator")
        .arg(Arg::with_name("IP Address")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("An IP address of the network")
        )
        .arg(Arg::with_name("Network mask")
            .required(true)
            .takes_value(true)
            .index(2)
            .help("Mask must be like 255.255.255.0")
        )
        .arg(Arg::with_name("Subnet mask")
            .required(true)
            .takes_value(true)
            .index(3)
            .help("Subnet-Mask must be like 255.255.255.0")
        )
        .get_matches();

    /* Default value */
    let mut ip = Ipv4Addr::new(192,168,0,1);
    let mut mask = Ipv4Addr::new(255,255,255,0);
    let mut subnet_mask = Ipv4Addr::new(255,255,255,192);


    match matches.value_of("IP Address") {
        Some(a) => {
            match a.parse::<Ipv4Addr>() {
                Ok(b) => { ip = b; },
                Err(_) => eprintln!("Non-valid IPv4")
            }
        },
        None => eprintln!("Error")
    }

    match matches.value_of("Network mask") {
        Some(a) => {
            match a.parse::<Ipv4Addr>() {
                Ok(b) => { mask = b; },
                Err(_) => eprintln!("Non-valid network mask")
            }
        },
        None => eprintln!("Error")
    }

    match matches.value_of("Subnet mask") {
        Some(a) => {
            match a.parse::<Ipv4Addr>() {
                Ok(b) => { subnet_mask = b; },
                Err(_) => eprintln!("Non-valid subnet mask")
            }
        },
        None => eprintln!("Error")
    }

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



