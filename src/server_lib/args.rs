use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[clap(
    name = "vtt_server",
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(default_value = "0.0.0.0:7768")]
    pub host: SocketAddr,
}
