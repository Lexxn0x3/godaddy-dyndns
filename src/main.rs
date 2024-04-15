
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use clap::Parser;
use std::thread;
use std::time::Duration;

mod arguments;

#[derive(Serialize)]
struct Record {
    data: String,
    ttl: u32,
}

#[derive(Deserialize)]
struct IpResponse {
    ip: String,
}

fn main() {

    let args = arguments::Args::parse();

    let domain = args.domain;
    let record_name = args.record_name;
    let api_key = args.api_key;
    let api_secret = args.api_secret;
    let client = Client::new();

    loop {
        // Configure the type of DNS record and desired IP type
        let record_type = "A"; // Choose "A" for IPv4 or "AAAA" for IPv6
        let ip_service = match record_type {
            "A" => "https://api64.ipify.org?format=json",
            "AAAA" => "https://api6.ipify.org?format=json",
            _ => panic!("Unsupported record type"),
        };

            // Step 1: Get the current public IP address
            let ip_res = client.get(ip_service)
            .send()
            .expect("Failed to send request for IP")
            .json::<IpResponse>()
            .expect("Failed to parse IP response");
        let ip_address = ip_res.ip;
        println!("public ip: {}", ip_address);
        // Step 2: Prepare the DNS record
        let records = vec![
            Record { data: ip_address.clone(), ttl: 3600 },  // TTL is set to 1 hour
        ];

        // Step 3: Update the DNS record
        let url = format!("https://api.godaddy.com/v1/domains/{}/records/{}/{}",
                       domain, record_type, record_name);
        let auth_header = format!("sso-key {}:{}", api_key, api_secret);
        client.put(&url)
            .header("Authorization", auth_header)
            .json(&records)
            .send()
            .expect("Failed to update DNS record");

        println!("Updated {} record for {} to IP: {}", record_type, record_name, ip_address);
    
        std::thread::sleep(Duration::from_secs(args.update_time))
    }
}
