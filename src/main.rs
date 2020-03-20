extern crate clap;

use std::net::Ipv4Addr;
use clap::{Arg, App};


fn network_address(ip: &Ipv4Addr, mask: &Ipv4Addr) -> Ipv4Addr {
    let mut network: Vec<u8> = Vec::with_capacity(4);

    let ip = ip.octets();
    let mask = mask.octets();

    for i in 0..4 {
        // Logical AND between IP and MASK
        let res = ip[i] & mask[i];

        network.push(res);
    }

    return Ipv4Addr::new(network[0], network[1], network[2], network[3])
}

fn wildcard_address(mask: &Ipv4Addr) -> Ipv4Addr{
    let mut wildcard: Vec<u8> = Vec::with_capacity(4);

    let mask = mask.octets();

    for i in 0..4 {
        // Logical NOT in MASK
        let res = !mask[i];

        wildcard.push(res);
    }

    return Ipv4Addr::new(wildcard[0], wildcard[1], wildcard[2], wildcard[3])
}

fn broadcast_address(ip: &Ipv4Addr, mask: &Ipv4Addr) -> Ipv4Addr {
    let mut broadcast: Vec<u8> = Vec::with_capacity(4);

    let ip = ip.octets();
    let wildcard = wildcard_address(&mask).octets();

    for i in 0..4 {
        // Logical NOT in MASK
        let res = ip[i] | wildcard[i];

        broadcast.push(res);
    }

    return Ipv4Addr::new(broadcast[0], broadcast[1], broadcast[2], broadcast[3])
}

fn first_address(network: &Ipv4Addr) -> Ipv4Addr {
    let network = network.octets();

    let last_octet = network[3] + 1;

    return Ipv4Addr::new(network[0], network[1], network[2], last_octet);
}

fn last_address(broadcast: &Ipv4Addr) -> Ipv4Addr{
    let broadcast = broadcast.octets();

    let last_octet = broadcast[3] - 1;

    return Ipv4Addr::new(broadcast[0], broadcast[1], broadcast[2], last_octet);
}

fn host_count(wildcard: &Ipv4Addr) -> u32 {
    let wildcard = wildcard.octets();

    let sum: u32 = vec_to_num(wildcard) - 1;

    return sum;
}

/* UTILS */

fn mask_to_cidr(mask: &Ipv4Addr) -> u8 {
    let mut mask = vec_to_num(mask.octets()) as f64;

    let mut count = 0;
    while mask % 2.0 == 0.0 {
        count += 1;
        mask = mask / 2_f64;
    }

    return 32 - count;
}

fn vec_to_num(vec: [u8; 4]) -> u32 {
    let a: u32 = (vec[0] as u32 * 16_777_216) as u32;
    let b: u32 = (vec[1] as u32 * 65536) as u32;
    let c: u32 = (vec[2] as u32 * 256) as u32;
    let d: u32 =  vec[3] as u32;

    return a + b + c + d;
}

fn num_to_vec(number: u32) -> [u8; 4] {
    let mut num = number;

    let a: u8 = (num % 256) as u8;
    num = num / 256;

    let b: u8 = (num % 256) as u8;
    num = num / 256;

    let c: u8 = (num % 256) as u8;
    num = num / 256;

    let d: u8 = (num % 256) as u8;


    return [d,c,b,a];
}

/* SUBNET CALCULATOR */
fn subnet_count(wildcard: &Ipv4Addr, subnet_wildcard: &Ipv4Addr) -> u32{
    let wildcard = vec_to_num(wildcard.octets());
    let subnet_wildcard = vec_to_num(subnet_wildcard.octets());

    return if wildcard == subnet_wildcard {
        0
    } else {
        (wildcard + 1) / (subnet_wildcard + 1)
    }


}

fn subnet_calc(network: &Ipv4Addr, mask: &Ipv4Addr, subnet_mask: &Ipv4Addr) -> Vec<Ipv4Addr> {
    let mut sub_network = vec_to_num(network.octets());
    let cidr = mask_to_cidr(mask);
    let subnet_cidr = mask_to_cidr(subnet_mask);

    let ecart = subnet_cidr - cidr;
    let pas = 2_u32.pow((32 - subnet_cidr).into());



    let mut subnet = Vec::new();

    for _ in 0..(2 ** &ecart) {
        let address = num_to_vec(sub_network);

        subnet.push(Ipv4Addr::new(address[0],address[1],address[2],address[3]));

        sub_network += pas ;
    }



    return subnet;

}


/* Menu */



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

    let network = network_address(&ip, &mask);
    let wildcard = wildcard_address(&mask);
    let broadcast = broadcast_address(&ip, &mask);
    let cidr = mask_to_cidr(&mask);
    let first = first_address(&network);
    let last = last_address(&broadcast);
    let hosts = host_count(&wildcard);

    println!();

    println!("{} /{}", ip, cidr);
    println!("\tNetwork : {:#?}", network);
    println!("\tBroadcast : {:#?}", broadcast);
    println!("\tMask : {:#?}", mask);
    println!("\tWilcard : {:#?}", wildcard);


    println!("\tAvailable  : {} - {}", first, last);
    println!("\tFree Hosts : {}", hosts);



    println!();

    let subnet_wildcard = wildcard_address(&subnet_mask);
    let subnet_count = subnet_count(&wildcard, &subnet_wildcard);

    println!("Subnet-mask : {}", subnet_mask);
    println!("Number of sub-network : {}", subnet_count);


    let subnetworks = subnet_calc(&network,&mask,&subnet_mask);

    for subnetwork in subnetworks {

        let cidr = mask_to_cidr(&subnet_mask);
        let broadcast = broadcast_address(&subnetwork,&subnet_mask);

        let first = first_address(&subnetwork);
        let last = last_address(&broadcast);

        let hosts = host_count(&wildcard_address(&subnet_mask));

        println!();
        println!("{} /{}", subnetwork, cidr);
        println!("\tAvailable  : {} - {}", first, last);
        println!("\tFree Hosts : {}", hosts);
        println!("\tBroadcast  : {}", broadcast);
    }


    println!();
}



