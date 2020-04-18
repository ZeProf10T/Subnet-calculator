extern crate clap;
use clap::{Arg, App};

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

pub mod ipv4;
pub mod ipv6;
pub mod export;
pub mod show;




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
        .arg(Arg::with_name("csv")
            .short("c")
            .long("csv")
            .takes_value(true)
            .value_name("FILE")
            .help("Create a CSV file based on subnetworks")

        )
        .get_matches();



    match matches.value_of("Version") {
        Some("4") => {
            let ip_values = matches.value_of("IP Address").unwrap();
            let mask_values = matches.value_of("Network mask").unwrap();
            let subnet_mask_values = matches.value_of("Subnet mask").unwrap();
            let mut ok = true;


            let mut ip: Ipv4Addr= Ipv4Addr::new(192,168,10, 1);
            let mut mask: Ipv4Addr = Ipv4Addr::new(255,255,255,0);
            let mut subnet_mask: Ipv4Addr = Ipv4Addr::new(255,255,255,252);


            match ip_values.parse::<Ipv4Addr>() {
                Ok(i) => { ip = i;},
                _ => {
                    eprintln!("Invalid IP");
                    ok = false;
                }
            }

            match ipv4::utils::valid_mask(mask_values) {
                Some(a) => { mask = a; },
                None => {
                    eprintln!("Invalid Mask");
                    ok = false;
                }

            }

            match ipv4::utils::valid_mask(subnet_mask_values) {
                Some(a) => {
                    subnet_mask = a;
                },
                None => {
                    eprintln!("Invalid Subnet-Mask");
                    ok = false;
                }
            }

            if ok {
                if matches.is_present("csv") {
                    let file = matches.value_of("csv").unwrap_or("subnetwork");
                    export::quatre(ip, mask, subnet_mask, file).unwrap_or_else(|e| eprintln!("Error in the creation of the file !,\n {}",e));;
                }
                else {
                    show::quatre(ip, mask, subnet_mask);
                }


            }



        },
        Some("6") => {
            let ip_values = matches.value_of("IP Address").unwrap();
            let mask_values = matches.value_of("Network mask").unwrap();
            let subnet_mask_values = matches.value_of("Subnet mask").unwrap();
            let mut ok = true;


            let mut ip = Ipv6Addr::new(0xfe80,0,0,1,0,0,0,1);
            let mut mask = Ipv6Addr::new(0xffff,0xffff,0xffff,0x0003,0,0,0,0);
            let mut subnet_mask = Ipv6Addr::new(0xffff,0xffff,0xffff,0,0,0,0,0);


            match ip_values.parse::<Ipv6Addr>() {
                Ok(b) => { ip = b; },
                Err(_) => {
                    eprintln!("Invalid IPv6");
                    ok = false;
                }
            }

            match ipv6::utils::valid_mask(mask_values) {
                Some(a) => { mask = a; },
                None => {
                    eprintln!("Invalid Mask");
                    ok = false;
                }

            }

            match ipv6::utils::valid_mask(subnet_mask_values) {
                Some(a) => {
                    subnet_mask = a;
                },
                None => {
                    eprintln!("Invalid Subnet-Mask");
                    ok = false;
                }
            }


            if ok {
                if matches.is_present("csv") {
                    let file = matches.value_of("csv").unwrap_or("subnetwork");
                    export::six(ip, mask, subnet_mask, file).unwrap_or_else(|e| eprintln!("Error in the creation of the file !,\n {}",e));
                }
                else {
                    show::six(ip, mask, subnet_mask);
                }


            }
        },
        _ => eprintln!("You must choose an IP version : 4 or 6")
    }





}

