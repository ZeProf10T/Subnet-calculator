use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::error::Error;

use crate::ipv4;
use crate::ipv6;

pub fn quatre(ip: Ipv4Addr, mask: Ipv4Addr, subnet_mask: Option<Ipv4Addr>) {

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

    if subnet_mask.is_some() {
        let subnet_mask = subnet_mask.unwrap();

        let subnet_wildcard = ipv4::address::wildcard(&subnet_mask);
        let subnet_count = ipv4::subnet::count(&wildcard, &subnet_wildcard);

        println!("Subnet-mask : {}", subnet_mask);
        println!("Number of sub-network : {}", subnet_count);


        let subnetworks = ipv4::subnet::calculate(&network, &mask, &subnet_mask);

        for subnetwork in &subnetworks {

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






}

pub fn six(ip: Ipv6Addr, mask: Ipv6Addr, subnet_mask: Option<Ipv6Addr>) {
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
    if subnet_mask.is_some() {
        let subnet_mask = subnet_mask.unwrap();
        let subnet_wildcard = ipv6::address::wildcard(&subnet_mask);
        let count = ipv6::subnet::count(&wildcard, &subnet_wildcard);


        println!();
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
}