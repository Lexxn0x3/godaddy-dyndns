
# 🌐 godaddy-dyndns
A Rust utility to keep your GoDaddy DNS records synchronized with your dynamic IP. Ideal for home servers and environments with dynamic IPs.

## 🚀 Features
- Update A or AAAA records on GoDaddy to your current IP.
- Supports both IPv4 and IPv6 addresses.
- Configurable update intervals.
- Secure API key and secret management.

## ⚙️ Usage

```
godaddy-dyndns [OPTIONS] --api-key <API_KEY> --api-secret <API_SECRET> --record-name <RECORD_NAME> --update-time <UPDATE_TIME> --domain <DOMAIN>
```

### Options
- `-i, --ip-v6` - Use IPv6 and AAAA records.
- `--api-key <API_KEY>` - Your API-Key.
- `--api-secret <API_SECRET>` - Your API-Secret.
- `-r, --record-name <RECORD_NAME>` - The name of the record to update.
- `-u, --update-time <UPDATE_TIME>` - Time in seconds between updates.
- `-d, --domain <DOMAIN>` - Domain to update.
- `-h, --help` - Print help information.
- `-V, --version` - Print version information.

## 🛠️ Getting Started

### Prerequisites
- Rust and Cargo installed on your machine.
- A GoDaddy account with API access.

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Lexxn0x3/godaddy-dyndns.git
   ```
2. Change into the project directory:
   ```bash
   cd godaddy-dyndns
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

### Configuration

#### Option 1: Using an `.env` File
Create a `.env` file in the project root and fill in your API key, secret, and other configurations:
```
API_KEY=your_api_key_here
API_SECRET=your_api_secret_here
RECORD_NAME=your_record_name
DOMAIN=your_domain.com
UPDATE_TIME=300  # time in seconds
```
#### Option 2: Command Line Arguments
Alternatively, you can run the program directly with command line arguments for temporary or one-time configuration needs.

### Running
For persistent configurations, run the program using:
```bash
cargo run --release
```
For one-time configurations, use:
```bash
cargo run --release --api-key your_key --api-secret your_secret --record-name "example" --update-time 600 --domain "example.com"
```

## 📋 Example
Update the DNS record for `example.com` every 600 seconds:
```bash
godaddy-dyndns --api-key <your_key> --api-secret <your_secret> --record-name <example> --update-time 600 --domain <example.com>
```

## 🤝 Contributing
Contributions, issues, and feature requests are welcome!

## ⭐️ Show Your Support
Give a ⭐️ if this project helped you!

## 🔒 License
Distributed under the GPL-2.0 License. See `LICENSE` for more information.
