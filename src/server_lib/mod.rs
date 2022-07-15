mod args;
pub mod server_start;
use clap::Parser;

pub async fn dedicated_server_start() {
    println!("Dedicated Server");
    let args = crate::server_lib::args::Args::parse();

    server_start::server_start(&args.host).await;
}