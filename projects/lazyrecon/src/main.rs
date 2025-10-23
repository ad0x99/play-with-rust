use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use clap::{Arg, Command};

use anyhow::Ok;
use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};

use crate::model::Subdomain;
pub use error::Error;
mod common_ports;
mod error;
mod model;
mod ports;
mod subdomains;

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("lazyrecon")
        .version("1.0")
        .author("Your Name <your@email.com>")
        .about("Multi-threaded subdomain and port scanner")
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("TARGET")
                .help("Target domain to scan")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Save results to a file instead of printing to console")
                .required(false),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Output format: plain or json (default: plain)")
                .required(false),
        )
        .get_matches();

    let target = matches.get_one::<String>("target").unwrap();
    let output_path = matches.get_one::<String>("output");
    let format = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("plain");

    println!("Scanning target: {}", target);

    // Set the HTTP request timeout duration to 5 seconds
    let http_timeout = Duration::from_secs(5);

    // Build a blocking HTTP client with a redirect policy (max 4 redirects) and the specified timeout
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    // Spinner setup
    let spinning = Arc::new(AtomicBool::new(true));
    let spinning_clone = Arc::clone(&spinning);
    let spinner_handle = thread::spawn(move || {
        let spinner_chars = ['|', '/', '-', '\\'];
        let mut i = 0;
        while spinning_clone.load(Ordering::Relaxed) {
            print!("\rLoading... {}", spinner_chars[i % spinner_chars.len()]);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
            i += 1;
        }
        print!("\r                \r"); // Clear the line after done
    });

    // Use a custom thread pool to improve speed
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    let results = pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();
        scan_result
    });

    let output = if format == "json" {
        // Serialize results as JSON
        serde_json::to_string_pretty(&results).unwrap()
    } else {
        // Format results as plain text
        let mut out = String::new();
        for subdomain in &results {
            out.push_str("\n==============================\n");
            out.push_str(&format!("Subdomain: {}\n", &subdomain.domain));
            out.push_str("------------------------------\n");
            if subdomain.open_ports.is_empty() {
                out.push_str("    No open ports found.\n");
            } else {
                out.push_str("    Open Ports:\n");
                for port in &subdomain.open_ports {
                    out.push_str(&format!("      - {}\n", port.port));
                }
            }
        }
        out.push_str("\n==============================\n");
        out
    };

    if let Some(path) = output_path {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path)?;
        file.write_all(output.as_bytes())?;
        println!("Results saved to {}", path);
    } else {
        print!("{}", output);
    }

    // Stop spinner and wait for thread to finish
    spinning.store(false, Ordering::Relaxed);
    spinner_handle.join().unwrap();

    println!("Scan complete!");

    Ok(())
}
