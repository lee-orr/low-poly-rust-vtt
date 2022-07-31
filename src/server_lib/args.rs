use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "vtt_server",
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(long, default_value = "7767")]
    pub port: u16,
    #[clap(long, default_value = "7768")]
    pub webrtc_port: u16,
    #[clap(long, default_value = "http://127.0.0.1:7768")]
    pub webrtc_url: String,
}
