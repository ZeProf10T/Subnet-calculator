extern crate csv;

use std::io;
use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr};
use csv::Writer;

use crate::ipv4;
use crate::ipv6;
use std::fmt::format;


pub fn quatre(ip: Ipv4Addr, mask: Ipv4Addr, subnet_mask: Ipv4Addr, file: &str) -> Result<(), Box<dyn Error>> {

    /* Computation */
    let network = ipv4::address::network(&ip, &mask);
    let wildcard = ipv4::address::wildcard(&mask);
    let broadcast = ipv4::address::broadcast(&ip, &mask);
    let cidr = ipv4::utils::mask_to_cidr(&mask);
    let first = ipv4::address::first(&network);
    let last = ipv4::address::last(&broadcast);
    let hosts = ipv4::utils::host_count(&wildcard);
    let subnet_wildcard = ipv4::address::wildcard(&subnet_mask);
    let subnet_count = ipv4::subnet::count(&wildcard, &subnet_wildcard);

    /* Subnetwork computation */
    let subnetworks = ipv4::subnet::calculate(&network, &mask, &subnet_mask);




    let mut wtr = Writer::from_path(format!("{}.csv",&file))?;

    wtr.write_record(&["Network", "Mask", "Broadcast", "First", "Last", "Hosts count"])?;

    for subnetwork in subnetworks {
        let cidr = ipv4::utils::mask_to_cidr(&subnet_mask);
        let broadcast = ipv4::address::broadcast(&subnetwork, &subnet_mask);

        let first = ipv4::address::first(&subnetwork);
        let last = ipv4::address::last(&broadcast);

        let hosts = ipv4::utils::host_count(&ipv4::address::wildcard(&subnet_mask));

        wtr.serialize((subnetwork,subnet_mask, broadcast, first, last, hosts))?;


    }

    wtr.flush()?;

    println!("{}.csv was successful created",&file);
    Ok(())
}

pub fn six(ip: Ipv6Addr, mask: Ipv6Addr, subnet_mask: Ipv6Addr, file: &str) -> Result<(), Box<dyn Error>>{

    let network = ipv6::address::network(&ip, &mask);
    let wildcard = ipv6::address::wildcard(&mask);
    let broadcast = ipv6::address::broadcast(&ip, &wildcard);

    let first = ipv6::address::first(&network);
    let last = ipv6::address::last(&broadcast);

    let hosts = ipv6::utils::host_count(&wildcard);
    let cidr = ipv6::utils::mask_to_cidr(&mask);

    /* SUBNETWORK */
    let subnet_wildcard = ipv6::address::wildcard(&subnet_mask);
    let count = ipv6::subnet::count(&wildcard, &subnet_wildcard);


    let subnetworks = ipv6::subnet::calculate(&network, &mask, &subnet_mask);

    let mut wtr = Writer::from_path(format!("{}.csv",&file))?;

    wtr.write_record(&["Network", "CIDR", "Broadcast", "First", "Last", "Hosts count"])?;


    for subnetwork in subnetworks {

        let subnet_cidr = ipv6::utils::mask_to_cidr(&subnet_mask);

        let first = ipv6::address::first(&subnetwork);
        let last = ipv6::address::last(&broadcast);

        let hosts = ipv6::utils::host_count(&ipv6::address::wildcard(&subnet_mask));

        wtr.serialize((subnetwork,subnet_cidr, broadcast, first, last, hosts))?;


    }

    wtr.flush()?;

    println!("{}.csv was successful created",&file);
    Ok(())

}