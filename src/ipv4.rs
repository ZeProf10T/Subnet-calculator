use std::net::Ipv4Addr;

pub mod address {
    use super::*;

    pub fn network(ip: &Ipv4Addr, mask: &Ipv4Addr) -> Ipv4Addr {
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

    pub fn wildcard(mask: &Ipv4Addr) -> Ipv4Addr{
        let mut wildcard: Vec<u8> = Vec::with_capacity(4);

        let mask = mask.octets();

        for i in 0..4 {
            // Logical NOT in MASK
            let res = !mask[i];

            wildcard.push(res);
        }

        return Ipv4Addr::new(wildcard[0], wildcard[1], wildcard[2], wildcard[3])
    }

    pub fn broadcast(ip: &Ipv4Addr, mask: &Ipv4Addr) -> Ipv4Addr {
        let mut broadcast: Vec<u8> = Vec::with_capacity(4);

        let ip = ip.octets();
        let wildcard = wildcard(&mask).octets();

        for i in 0..4 {
            // Logical NOT in MASK
            let res = ip[i] | wildcard[i];

            broadcast.push(res);
        }

        return Ipv4Addr::new(broadcast[0], broadcast[1], broadcast[2], broadcast[3])
    }

    pub fn first(network: &Ipv4Addr) -> Ipv4Addr {
        let network = network.octets();

        let last_octet = network[3] + 1;

        return Ipv4Addr::new(network[0], network[1], network[2], last_octet);
    }

    pub fn last(broadcast: &Ipv4Addr) -> Ipv4Addr{
        let broadcast = broadcast.octets();

        let last_octet = broadcast[3] - 1;

        return Ipv4Addr::new(broadcast[0], broadcast[1], broadcast[2], last_octet);
    }
}

pub mod subnet {
    use super::*;

    pub fn count(wildcard: &Ipv4Addr, subnet_wildcard: &Ipv4Addr) -> u32{
        let wildcard = utils::vec_to_num(wildcard.octets());
        let subnet_wildcard = utils::vec_to_num(subnet_wildcard.octets());

        return if wildcard == subnet_wildcard {
            0
        } else {
            (wildcard + 1) / (subnet_wildcard + 1)
        }


    }

    pub fn calculate(network: &Ipv4Addr, mask: &Ipv4Addr, subnet_mask: &Ipv4Addr) -> Vec<Ipv4Addr> {
        let mut sub_network = utils::vec_to_num(network.octets());
        let cidr = utils::mask_to_cidr(mask);
        let subnet_cidr = utils::mask_to_cidr(subnet_mask);

        let ecart = subnet_cidr - cidr;
        let pas = 2_u32.pow((32 - subnet_cidr).into());


        let mut subnet = Vec::new();

        for _ in 0..(2 ** &ecart) {
            let address = utils::num_to_vec(sub_network);

            subnet.push(Ipv4Addr::new(address[0],address[1],address[2],address[3]));

            sub_network += pas ;
        }



        return subnet;

    }
}


pub mod utils {
    use super::*;

    pub fn mask_to_cidr(mask: &Ipv4Addr) -> u8 {
        let mut mask = vec_to_num(mask.octets()) as f64;

        let mut count = 0;
        while mask % 2.0 == 0.0 {
            count += 1;
            mask = mask / 2_f64;
        }

        return 32 - count;
    }

    pub fn cidr_to_mask(cidr: u8) -> Ipv4Addr {
        let mut mask: u32 = 0;
        let mut n = 2_147_483_648;

        for _i in 0..cidr {
            mask += n;
            n = n / 2;
        }

        let res = num_to_vec(mask as u32);
        return Ipv4Addr::new(res[0],res[1],res[2],res[3]);
    }

    pub fn vec_to_num(vec: [u8; 4]) -> u32 {
        let a: u32 = (vec[0] as u32 * 16_777_216) as u32;
        let b: u32 = (vec[1] as u32 * 65536) as u32;
        let c: u32 = (vec[2] as u32 * 256) as u32;
        let d: u32 =  vec[3] as u32;

        return a + b + c + d;
    }

    pub fn num_to_vec(number: u32) -> [u8; 4] {
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

    pub fn host_count(wildcard: &Ipv4Addr) -> u32 {
        let wildcard = wildcard.octets();

        let sum: u32 = vec_to_num(wildcard) - 1;

        return sum;
    }

    pub fn valid_mask(mask: &str) -> Option<Ipv4Addr>{
        match mask.parse::<Ipv4Addr>() {
            Ok(a) => {
                return Some(a)
            },
            Err(_) => { }
        }

        match mask.parse::<u8>() {
            Ok(a) => {
                return Some(cidr_to_mask(a))
            },
            Err(_) => {}
        }


        return None;
    }



}