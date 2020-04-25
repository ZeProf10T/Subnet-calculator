extern crate clap;
use clap::{load_yaml, App};

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

pub mod ipv4;
pub mod ipv6;
pub mod export;
pub mod show;
pub mod common;

fn main() {

    /* Initiate clap application */
    let yml = load_yaml!("app.yml");
    let matches = App::from(yml).get_matches();



    /*
    ** GET values from clap
    */

    let address = matches.value_of("address").unwrap();
    let ip_values = matches.value_of("address").unwrap();
    let mask_values = matches.value_of("netmask").unwrap();

    let subnet_mask_values: Option<&str> = match matches.is_present("subnetmask") {
        true => Some(matches.value_of("subnetmask").unwrap()),
        false => None
    };

    let file = match matches.is_present("csv") {
        true => Some( matches.value_of("csv").unwrap_or("subnetwork")),
        false => None
    };

    let limit: Option<u32> = match matches.is_present("limit") {
        true => match matches.value_of("limit").unwrap_or("8").parse() {
            Ok(a) => Some(a),
            _ => {
                eprintln!("Invalid limit, limit set to 8");
                Some(8_u32)
            }
        },
        false => None
    };



    /*
    ** MAIN program
    */

    match common::version(address) {

        Ok(common::Version::Four) => {


            /* Test variables formats */
            let ip= ip_values.parse::<Ipv4Addr>().ok();
            let mask =  ipv4::utils::valid_mask(mask_values);


            /* Test if subnet mask is valid */
            let subnet_mask = match subnet_mask_values {
                Some(a) => match ipv4::utils::valid_mask(a) {
                    Some(a) => Some(a),
                    None => {
                        eprintln!("Invalid subnet mask");
                        None
                    }
                },
                None => None
            };


            /* ERROR message */
            if ip.is_none() { eprintln!("Invalid address") }
            if mask.is_none() { eprintln!("Invalid mask") }


            /* If all work, execute the computing */
            if ip.is_some() && mask.is_some(){
                common::four(ip.unwrap(), mask.unwrap(), subnet_mask,file,limit).unwrap_or_else(|e| eprintln!("Error :\n{}", e));
            }

        },
        Ok(common::Version::Six) => {
            /* Test variables formats */
            let ip= ip_values.parse::<Ipv6Addr>().ok();
            let mask =  ipv6::utils::valid_mask(mask_values);


            /* Test if subnet mask is valid */
            let subnet_mask = match subnet_mask_values {
                Some(a) => match ipv6::utils::valid_mask(a) {
                    Some(a) => Some(a),
                    None => {
                        eprintln!("Invalid subnet mask");
                        None
                    }
                },
                None => None
            };


            /* ERROR message */
            if ip.is_none() { eprintln!("Invalid address") }
            if mask.is_none() { eprintln!("Invalid mask") }


            /* If all work, execute the computing */
            if ip.is_some() && mask.is_some(){
                common::six(ip.unwrap(), mask.unwrap(), subnet_mask,file,limit).unwrap_or_else(|e| eprintln!("Error :\n{}", e));
            }

            /*
            let mut ok = true;


            let mut ip = Ipv6Addr::new(0xfe80,0,0,1,0,0,0,1);
            let mut mask = Ipv6Addr::new(0xffff,0xffff,0xffff,0x0003,0,0,0,0);
            let mut subnet_mask = Some(Ipv6Addr::new(0xffff,0xffff,0xffff,0,0,0,0,0));


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

            if subnet_mask_values.is_some() {
                match ipv6::utils::valid_mask(subnet_mask_values.unwrap()) {
                    Some(a) => {
                        subnet_mask = Some(a);
                    },
                    None => {
                        eprintln!("Invalid Subnet-Mask");
                        ok = false;
                    }
                }
            } else {
                subnet_mask = None;
            }



            if ok {
                if matches.is_present("csv") {
                    let file = matches.value_of("csv").unwrap_or("subnetwork");
                    export::six(ip, mask, subnet_mask.unwrap(), file).unwrap_or_else(|e| eprintln!("Error in the creation of the file !,\n {}",e));
                }
                else {
                    show::six(ip, mask, subnet_mask);
                }


            }

            */
        },
        _ => eprintln!("You must have a valid IP address")
    }

}

