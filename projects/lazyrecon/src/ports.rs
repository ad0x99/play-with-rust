use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};
use rayon::prelude::*;
use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

/// Scans the most common ports on the given subdomain and updates its open_ports field.
/// Returns the updated Subdomain.
pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    // Resolve the subdomain to a socket address using port 1024 as a placeholder.
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    // If no socket addresses could be resolved, return the subdomain unchanged.
    if socket_addresses.len() == 0 {
        return subdomain;
    }

    // Scan the most common ports in parallel.
    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scan_port(socket_addresses[0], *port))
        .filter(|port| port.is_open) // Keep only open ports.
        .collect();

    subdomain
}

/// Attempts to connect to the given socket address at the specified port.
/// Returns a Port struct indicating if the port is open.
fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    // Set a timeout duration for the TCP connection attempt.
    let timeout = Duration::from_secs(3);

    // Update the socket address to use the specified port.
    socket_address.set_port(port);

    // Try to connect to the address within the timeout; check if the port is open.
    let is_open = TcpStream::connect_timeout(&socket_address, timeout).is_ok();

    Port { port, is_open }
}
