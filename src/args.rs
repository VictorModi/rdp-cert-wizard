use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rdp-cert-wizard")]
#[command(about = "Auto import and bind RDP TLS Cert", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub cert: String,

    #[arg(short, long)]
    pub key: String,

    #[arg(short = 'p', long)]
    pub password: Option<String>,

    #[arg(short = 'r', long)]
    pub restart: Option<bool>,
}
