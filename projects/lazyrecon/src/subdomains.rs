use std::time::Duration;

use reqwest::blocking::Client;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

use crate::error::Error;
use crate::model::{CrtShEntry, Subdomain};

/// Enumerates subdomains for the given target using crt.sh and filters valid ones.
/// Returns a vector of Subdomain structs that resolve via DNS.
pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    // Query crt.sh for certificates related to the target domain in JSON format
    let url = format!("https://crt.sh/?q=%25.{}&output=json", target);
    println!("Querying crt.sh with URL: {}", url);
    let entries: Vec<CrtShEntry> = http_client.get(&url).send()?.json()?;

    // Extract all subdomains from the crt.sh entries
    let extracted_subdomains: Vec<String> = entries
        .into_iter()
        .flat_map(|entry| {
            // Each entry may contain multiple subdomains separated by newlines
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    // Filter out the target itself and any wildcard subdomains
    let mut filtered_subdomains: Vec<String> = extracted_subdomains
        .into_iter()
        .filter(|subdomain| subdomain != target)
        .filter(|subdomain| !subdomain.contains('*'))
        .collect();

    // Add the target domain itself to the list
    filtered_subdomains.push(target.to_string());

    // Convert to Subdomain structs and keep only those that resolve via DNS
    let subdomains: Vec<Subdomain> = filtered_subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter(resolves)
        .collect();

    Ok(subdomains)
}

/// Checks if the given subdomain resolves to an IP address using DNS lookup.
/// Returns true if the domain resolves, false otherwise.
fn resolves(domain: &Subdomain) -> bool {
    // Set DNS resolver options with a 4-second timeout
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    // Create a new DNS resolver with default configuration and custom options
    let dns_resolver = Resolver::new(ResolverConfig::default(), opts)
        .expect("subdomain resolver: building DNS client");

    // Attempt to resolve the domain to an IP address
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
