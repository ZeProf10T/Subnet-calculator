use std::net::Ipv6Addr;

pub mod address {
    use super::*;

    pub fn network(ip: &Ipv6Addr, mask: &Ipv6Addr) -> Ipv6Addr {
        let mut network: Vec<u16> = Vec::with_capacity(4);

        let ip = ip.octets();
        let mask = mask.octets();

        for i in 0..16{
            if i % 2 == 0 {
                // Logical AND between IP and MASK
                let a: u16 = (ip[i] & mask[i]) as u16;
                let b: u16 = (ip[i + 1] & mask[i + 1]) as u16;

                /* Make one hextet with 2 bytes */
                let res = a * 256 + b;

                network.push(res);
            }

        }


        return Ipv6Addr::new(network[0],network[1],network[2],network[3],network[4],network[5],network[6],network[7]);
    }

    pub fn wildcard(mask: &Ipv6Addr) -> Ipv6Addr{
        let mut wildcard: Vec<u16> = Vec::with_capacity(4);

        let mask = mask.octets();

        for i in 0..16 {
            if i % 2 == 0 {
                // Logical AND between IP and MASK
                let a: u16 = !mask[i] as u16;
                let b: u16 = !mask[i + 1] as u16;
                // Logical NOT in MASK
                let res = a * 256 + b;

                wildcard.push(res);
            }
        }

        return Ipv6Addr::new(wildcard[0], wildcard[1], wildcard[2], wildcard[3],wildcard[4],wildcard[5], wildcard[6], wildcard[7])
    }


    pub fn first(network: &Ipv6Addr) -> Ipv6Addr {
        let network = network.octets();

        let mut res = Vec::new();

        for i in 0..16 {
            if i % 2 == 0 {
                let a: u16 = network[i] as u16;
                let b: u16 = network[i + 1] as u16;
                // Logical NOT in MASK
                res.push(a * 256 + b);
            }
        }
        let last_octet = res[7] + 1;

        return Ipv6Addr::new(res[0], res[1], res[2], res[3], res[4], res[5], res[6], last_octet);
    }

    /* This function have no sense with IPV6 */
    pub fn broadcast(ip: &Ipv6Addr, wildcard: &Ipv6Addr) -> Ipv6Addr {
        let mut broadcast: Vec<u16> = Vec::new();

        let ip = ip.octets();
        let wildcard = wildcard.octets();

        for i in 0..16 {
            // Logical NOT in MASK
            if i % 2 == 0 {
                let a: u16 = (ip[i] | wildcard[i]) as u16;
                let b: u16 = (ip[i + 1] | wildcard[i + 1]) as u16;
                broadcast.push(a * 256 + b);
            }

        }

        return Ipv6Addr::new(broadcast[0], broadcast[1], broadcast[2], broadcast[3], broadcast[4],broadcast[5], broadcast[6], broadcast[7]);
    }


    pub fn last(broadcast: &Ipv6Addr) -> Ipv6Addr{
        let mut res: Vec<u16> = Vec::new();
        let broadcast = broadcast.octets();

        for i in 0..16 {
            if i % 2 == 0 {
                let a: u16 = broadcast[i] as u16;
                let b: u16 = broadcast[i + 1] as u16;
                res.push(a * 256 + b);
            }
        }
        let last_octet = res[7] - 1;

        return Ipv6Addr::new(res[0], res[1], res[2],res[3], res[4], res[5], res[6],last_octet);
    }

}

pub mod subnet {
    use super::*;


    pub fn count(wildcard: &Ipv6Addr, subnet_wildcard: &Ipv6Addr) -> u128{
        let wildcard = wildcard.octets();
        let mut res = Vec::new();

        for i in 0..16 {

            if i % 2 == 0 {
                let a: u16 = wildcard[i] as u16;
                let b: u16 = wildcard[i + 1] as u16;
                res.push(a * 256 + b);
            }

        }

        let x: [u16; 8]= [res[0],res[1],res[2],res[3],res[4],res[5],res[6],res[7]];

        let subnet_wildcard = subnet_wildcard.octets();
        let mut res = Vec::new();

        for i in 0..16 {

            if i % 2 == 0 {
                let a: u16 = subnet_wildcard[i] as u16;
                let b: u16 = subnet_wildcard[i + 1] as u16;
                res.push(a * 256 + b);
            }

        }

        let y: [u16; 8]= [res[0],res[1],res[2],res[3],res[4],res[5],res[6],res[7]];

        let wildcard = utils::vec_to_num(x);
        let subnet_wildcard = utils::vec_to_num(y);

        return if wildcard == subnet_wildcard {
            0
        } else {
            (wildcard + 1) / (subnet_wildcard + 1)
        }


    }

    pub fn calculate(network: &Ipv6Addr, mask: &Ipv6Addr, subnet_mask: &Ipv6Addr) -> Vec<Ipv6Addr> {
        let mut res = Vec::new();
        let sub_network = network.octets();

        for i in 0..16 {

            if i % 2 == 0 {
                let a: u16 = sub_network[i] as u16;
                let b: u16 = sub_network[i + 1] as u16;
                res.push(a * 256 + b);
            }

        }

        let x: [u16; 8]= [res[0],res[1],res[2],res[3],res[4],res[5],res[6],res[7]];

        let mut sub_network = utils::vec_to_num(x);


        let cidr = utils::mask_to_cidr(mask);
        let subnet_cidr = utils::mask_to_cidr(subnet_mask);

        let ecart = subnet_cidr - cidr;
        let pas = 2_u128.pow((128 - subnet_cidr).into());


        let mut subnet = Vec::new();

        for _ in 0..(2 ** &ecart) {
            let address = utils::num_to_vec(sub_network);

            subnet.push(Ipv6Addr::new(address[0],address[1],address[2],address[3], address[4], address[5], address[6], address[7]));

            sub_network += pas ;
        }



        return subnet;

    }

}



pub mod utils {
    use super::*;


    pub fn mask_to_cidr(mask: &Ipv6Addr) -> u16 {
        let mut res: Vec<u16> = Vec::new();
        let mask = mask.octets();


        for i in 0..16 {

            if i % 2 == 0 {
                let a: u16 = mask[i] as u16;
                let b: u16 = mask[i + 1] as u16;
                res.push(a * 256 + b);
            }

        }

        let a: [u16; 8]= [res[0],res[1],res[2],res[3],res[4],res[5],res[6],res[7]];
        let mut mask = vec_to_num(a);

        let mut count = 0;

        while mask % 2 == 0 {
            count += 1;
            mask = mask / 2;
        }

        return 128 - count;
    }

    pub fn cidr_to_mask(cidr: u8) -> Ipv6Addr {
        let mut mask: u128 = 0;
        let mut n = 4_294_967_296 * 4_294_967_296 * 4_294_967_296 * 2_147_483_648;

        for _i in 0..cidr {
            mask += n;
            n = n / 2;
        }


        let add = num_to_vec(mask);
        return Ipv6Addr::new(add[0],add[1], add[2], add[3],add[4], add[5], add[6], add[7]);
    }



    pub fn vec_to_num(vec: [u16; 8]) -> u128 {
        let a: u128 = (vec[0] as u128 * 65536 * 65536 * 65536 * 65536 * 65536 * 65536 * 65536) as u128;
        let b: u128 = (vec[1] as u128 * 65536 * 65536 * 65536 * 65536 * 65536 * 65536)  as u128;
        let c: u128 = (vec[2] as u128 * 65536 * 65536 * 65536 * 65536 * 65536) as u128 ;
        let d: u128 = (vec[3] as u128 * 65536 * 65536 * 65536 * 65536) as u128;
        let e: u128 = (vec[4] as u128 * 65536 * 65536 * 65536) as u128;
        let f: u128 = (vec[5] as u128 * 65536 * 65536) as u128;
        let g: u128 = (vec[6] as u128 * 65536) as u128;
        let h: u128 = vec[7] as u128;

        return a + b + c + d + e + f + g + h;
    }


    pub fn num_to_vec(number: u128) -> [u16; 8] {
        let mut num = number;

        let a: u16 = (num % 65536) as u16;
        num = num / 65536;

        let b: u16 = (num % 65536) as u16;
        num = num / 65536;

        let c: u16 = (num % 65536) as u16;
        num = num / 65536;

        let d: u16 = (num % 65536) as u16;
        num = num / 65536;

        let e: u16 = (num % 65536) as u16;
        num = num / 65536;

        let f: u16 = (num % 65536) as u16;
        num = num / 65536;

        let g: u16 = (num % 65536) as u16;
        num = num / 65536;

        let h: u16 = (num % 65536) as u16;

        return [h,g,f,e,d,c,b,a];
    }

    pub fn host_count(wildcard: &Ipv6Addr) -> u128 {
        let mut res: Vec<u16> = Vec::with_capacity(8);
        let wildcard = wildcard.octets();



        for i in 0..16 {

            if i % 2 == 0 {
                let a: u16 = wildcard[i] as u16;
                let b: u16 = wildcard[i + 1] as u16;
                res.push(a * 256 + b);
            }

        }

        let a: [u16; 8]= [res[0],res[1],res[2],res[3],res[4],res[5],res[6],res[7]];

        let sum: u128 = vec_to_num(a) - 1;

        return sum;
    }


    pub fn valid_mask(mask: &str) -> Option<Ipv6Addr>{
         match mask.parse::<u8>() {
            Ok(a) => {
                return Some(cidr_to_mask(a))
            },
            Err(_) => {}
        }


        return None;
    }
 }
