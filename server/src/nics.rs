use nix::ifaddrs::{self, InterfaceAddress};
use nix::net::if_::InterfaceFlags;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct NicSummary {
    pub name: String,
    pub mac_address: Option<String>,

    /// Pairs of address/net-mask
    pub ipv4_addresses: Vec<(Ipv4Addr, Ipv4Addr)>,
}

impl NicSummary {
    pub fn new(name: String) -> Self {
        Self {
            name,
            mac_address: None,
            ipv4_addresses: vec![]
        }
    }
}

impl Display for NicSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"NicSummary(name: "{}", ipv4: {:#?}), mac-address: {:?}"#, self.name, self.ipv4_addresses, self.mac_address)
    }
}

fn is_serving(ifaddr: &InterfaceAddress) -> bool {
    !ifaddr.flags.contains(InterfaceFlags::IFF_LOOPBACK) &&
     ifaddr.flags.contains(InterfaceFlags::IFF_UP)
}

pub fn summarise() -> Result<Vec<NicSummary>, Box<dyn Error>> {
    let mut nics_by_name = HashMap::new();

    ifaddrs::getifaddrs()?
        .filter(is_serving)
        .for_each(|ifaddr| {

        let mut summary = nics_by_name
            .entry(ifaddr.interface_name.clone())
            .or_insert_with(|| NicSummary::new(ifaddr.interface_name));

        if let Some(address) = ifaddr.address {
            if let Some(mac_address) = address.as_link_addr() {
                summary.mac_address = mac_address.addr().map(format_mac);
            } else if let Some(sockaddr_in) = address.as_sockaddr_in() {
                let ip_address = Ipv4Addr::from(sockaddr_in.ip());
                let netmask = Ipv4Addr::from(ifaddr.netmask.expect("Cannot get netmask").as_sockaddr_in().unwrap().ip());

                summary.ipv4_addresses.push((ip_address, netmask))
            }
        }
    });

    let result: Vec<NicSummary> = nics_by_name.values()
        .filter(|nic| !nic.ipv4_addresses.is_empty())
        .cloned()
        .collect();

        Ok(result)
}

/// Build an iterator over the IPv4, non-loppback addresses
pub fn get_v4_addresses() -> impl Iterator<Item = Ipv4Addr> {
    ifaddrs::getifaddrs()
        .expect("Cannot get NICs")
        .filter_map(|ifaddr| if is_serving(&ifaddr) {None} else {ifaddr.address})
        .filter_map(|sockaddrstorage| sockaddrstorage.as_sockaddr_in().map(|s| Ipv4Addr::from(s.ip())))
}

fn format_mac(bytes: [u8; 6]) -> String {
    format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

/// Count the set bits in a byte using Brian Kerningham's algorithm.
fn count_ones(mut value: u8) -> u8 {
    let mut result = 0;
    while value > 0 {
        value &= value -1;
        result += 1;
    }
    result
}

/// Count the number of set bits in an Ipv4Addr
pub fn netmask_width(netmask: &Ipv4Addr) -> i32 {
    netmask.octets().iter().map(|x| count_ones(*x) as i32).sum()
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_netmask_width() {
        assert_eq!(0, netmask_width(&Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(8, netmask_width(&Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(5, netmask_width(&Ipv4Addr::new(1, 2, 3, 4)));
        assert_eq!(24, netmask_width(&Ipv4Addr::new(255, 255, 255, 0)));
        assert_eq!(16, netmask_width(&Ipv4Addr::new(255, 255, 0, 0)));
        assert_eq!(8, netmask_width(&Ipv4Addr::new(255, 0, 0, 0)));
    }

    #[test]
    pub fn test_count_ones() {
        assert_eq!(count_ones(0), 0);
        assert_eq!(count_ones(0b11111111), 8);
        assert_eq!(count_ones(0b10001000), 2);
        assert_eq!(count_ones(0b10001000), 2);
    }

}