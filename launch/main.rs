use clap::Parser;
use re_core::{options::with_port, runtime};
use tokio::signal;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Flags {
   #[arg(short, long, default_value_t=7777)]
   port: i32,
}

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    let args = Flags::parse();
    runtime::run(&[with_port(args.port)], signal::ctrl_c());
}
