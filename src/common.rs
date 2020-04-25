use std::net::{Ipv4Addr,Ipv6Addr};
use crate::{ipv4,ipv6};
use csv::Writer;
use std::error::Error;




pub fn four(ip: Ipv4Addr, mask: Ipv4Addr, subnet_mask: Option<Ipv4Addr>, export: Option<&str>, limit: Option<u32>) -> Result<(), Box<dyn Error>> {

    /*
    ** NETWORK INFORMATION
    */

    /* COMPUTING network */
    let network = ipv4::address::network(&ip, &mask);
    let wildcard = ipv4::address::wildcard(&mask);
    let broadcast = ipv4::address::broadcast(&ip, &mask);
    let cidr = ipv4::utils::mask_to_cidr(&mask);
    let first = ipv4::address::first(&network);
    let last = ipv4::address::last(&broadcast);
    let hosts = ipv4::utils::host_count(&wildcard);

    /* SHOW network*/
    println!();
    println!("{} /{}", ip, cidr);
    println!("\tNetwork : {:#?}", network);
    println!("\tBroadcast : {:#?}", broadcast);
    println!("\tMask : {:#?}", mask);
    println!("\tWilcard : {:#?}", wildcard);
    println!("\tAvailable  : {} - {}", first, last);
    println!("\tFree Hosts : {}", hosts);
    println!();


    /*
    ** SUBNETWORKS INFORMATION
    */

    if subnet_mask.is_some() {

        /* COMPUTING subnet */
        let subnet_mask = subnet_mask.unwrap();
        let subnet_wildcard = ipv4::address::wildcard(&subnet_mask);
        let subnet_count = ipv4::subnet::count(&wildcard, &subnet_wildcard);
        let subnetworks = ipv4::subnet::calculate(&network, &mask, &subnet_mask);

        /* SHOW subnet information */
        println!("Subnet-mask : {}", subnet_mask);
        println!("Number of sub-network : {}", subnet_count);


        /* EXPORT CSV */
        if export.is_some() {
            let file = export.unwrap_or("network");

            /* CREATE file */
            let mut wtr = Writer::from_path(format!("{}.csv",&file))?;
            wtr.write_record(&["Network", "Mask", "Broadcast", "First", "Last", "Hosts count"])?;


            for subnetwork in subnetworks {

                /* COMPUTING */
                let broadcast = ipv4::address::broadcast(&subnetwork, &subnet_mask);
                let first = ipv4::address::first(&subnetwork);
                let last = ipv4::address::last(&broadcast);
                let hosts = ipv4::utils::host_count(&ipv4::address::wildcard(&subnet_mask));

                /* ADD line to csv file */
                wtr.serialize((subnetwork,subnet_mask, broadcast, first, last, hosts))?;

            }


            /* CLOSE file */
            wtr.flush()?;
            println!("{}.csv was successful created",&file);


        } else {
            for subnetwork in subnetworks {

                /* COMPUTING */
                let cidr = ipv4::utils::mask_to_cidr(&subnet_mask);
                let broadcast = ipv4::address::broadcast(&subnetwork, &subnet_mask);
                let first = ipv4::address::first(&subnetwork);
                let last = ipv4::address::last(&broadcast);
                let hosts = ipv4::utils::host_count(&ipv4::address::wildcard(&subnet_mask));

                /* SHOW */
                println!();
                println!("{} /{}", subnetwork, cidr);
                println!("\tAvailable  : {} - {}", first, last);
                println!("\tFree Hosts : {}", hosts);
                println!("\tBroadcast  : {}", broadcast);

            }
        }


    }

    Ok(())
}


pub fn six(ip: Ipv6Addr, mask: Ipv6Addr, subnet_mask: Option<Ipv6Addr>, export: Option<&str>, limit: Option<u32>) -> Result<(), Box<dyn Error>> {

    /*
    ** NETWORK INFORMATION
    */

    /* COMPUTING network */
    let network = ipv6::address::network(&ip, &mask);
    let wildcard = ipv6::address::wildcard(&mask);
    let broadcast = ipv6::address::broadcast(&ip, &mask);
    let cidr = ipv6::utils::mask_to_cidr(&mask);
    let first = ipv6::address::first(&network);
    let last = ipv6::address::last(&broadcast);
    let hosts = ipv6::utils::host_count(&wildcard);

    /* SHOW network*/
    println!();
    println!("{} /{}", ip, cidr);
    println!("\tNetwork : {:#?}", network);
    println!("\tBroadcast : {:#?}", broadcast);
    println!("\tMask : {:#?}", mask);
    println!("\tWilcard : {:#?}", wildcard);
    println!("\tAvailable  : {} - {}", first, last);
    println!("\tFree Hosts : {}", hosts);
    println!();


    /*
    ** SUBNETWORKS INFORMATION
    */

    if subnet_mask.is_some() {

        /* COMPUTING subnet */
        let subnet_mask = subnet_mask.unwrap();
        let subnet_wildcard = ipv6::address::wildcard(&subnet_mask);
        let subnet_count = ipv6::subnet::count(&wildcard, &subnet_wildcard);
        let subnetworks = ipv6::subnet::calculate(&network, &mask, &subnet_mask);

        /* SHOW subnet information */
        println!("Subnet-mask : {}", subnet_mask);
        println!("Number of sub-network : {}", subnet_count);


        /* EXPORT CSV */
        if export.is_some() {
            let file = export.unwrap_or("network");

            /* CREATE file */
            let mut wtr = Writer::from_path(format!("{}.csv",&file))?;
            wtr.write_record(&["Network", "CIDR", "Broadcast", "First", "Last", "Hosts count"])?;


            for subnetwork in subnetworks {

                /* COMPUTING */
                let subnet_cidr = ipv6::utils::mask_to_cidr(&subnet_mask);
                let first = ipv6::address::first(&subnetwork);
                let last = ipv6::address::last(&broadcast);
                let hosts = ipv6::utils::host_count(&ipv6::address::wildcard(&subnet_mask));

                /* ADD line to csv file */
                wtr.serialize((subnetwork,subnet_cidr, broadcast, first, last, hosts))?;


            }


            /* CLOSE file */
            wtr.flush()?;
            println!("{}.csv was successful created",&file);


        } else {
            for subnetwork in subnetworks {

                /* COMPUTING */
                let cidr = ipv6::utils::mask_to_cidr(&subnet_mask);
                let broadcast = ipv6::address::broadcast(&subnetwork, &subnet_mask);
                let first = ipv6::address::first(&subnetwork);
                let last = ipv6::address::last(&broadcast);
                let hosts = ipv6::utils::host_count(&ipv6::address::wildcard(&subnet_mask));

                /* SHOW */
                println!();
                println!("{} /{}", subnetwork, cidr);
                println!("\tAvailable  : {} - {}", first, last);
                println!("\tFree Hosts : {}", hosts);
                println!("\tBroadcast  : {}", broadcast);

            }
        }


    }

    Ok(())
}



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