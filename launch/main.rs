use clap::Parser;
use re_core::{options::with_port, runtime};
use time::macros::format_description;
use tokio::signal;
use tracing_subscriber::{fmt::time::LocalTime, EnvFilter, FmtSubscriber};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Flags {
    #[arg(short, long, default_value_t = 7777)]
    port: i32,
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("debug"))
        .with_timer(LocalTime::new(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        )))
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let args = Flags::parse();
    runtime::run(&[with_port(args.port)], signal::ctrl_c());
}
