use clap::{Arg, App};
use ipnetwork::IpNetwork;
use std::net::IpAddr;

fn main() {
    let matches = App::new("buffer")
        .version("1.0")
        .author("Michael Mendy <montana@linux.com>")
        .about("Checks if an IP address is within a subnet")
        .arg(
            Arg::new("ip")
                .about("IP address to check")
                .required(true)
                .multiple_values(true)
                .index(1),
        )
        .arg(
            Arg::new("subnet")
                .about("Subnet to check against")
                .required(true)
                .multiple_values(true)
                .index(2),
        )
        .get_matches();

    let ip_strs: Vec<&str> = matches.values_of("ip").unwrap().collect();
    let subnet_strs: Vec<&str> = matches.values_of("subnet").unwrap().collect();

    for ip_str in ip_strs {
        let ip: IpAddr = match ip_str.parse() {
            Ok(ip) => ip,
            Err(_) => {
                eprintln!("Invalid IP address format: {}", ip_str);
                continue;
            }
        };

        for subnet_str in &subnet_strs {
            let subnet: IpNetwork = match subnet_str.parse() {
                Ok(subnet) => subnet,
                Err(_) => {
                    eprintln!("Invalid subnet format: {}", subnet_str);
                    continue;
                }
            };

            if subnet.contains(ip) {
                println!("IP {} is within the subnet {}", ip_str, subnet_str);
            } else {
                println!("IP {} is NOT within the subnet {}", ip_str, subnet_str);
            }
        }
    }
}
