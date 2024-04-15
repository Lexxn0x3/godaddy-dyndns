use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Use V6 and AAAA records
    #[arg(short, long, default_value_t = false)]
    pub ip_v6: bool,

    /// Your API-Key
    #[arg(long)]
    pub api_key: String,
    
    /// Your API-Secret
    #[arg(long)]
    pub api_secret: String,

    /// The name of the record
    #[arg(short, long)]
    pub record_name: String,

    /// Time between updates
    #[arg(short, long)]
    pub update_time: u64,

    /// Domain to update
    #[arg(short, long)]
    pub domain: String,
}
