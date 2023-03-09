use re_core::{options::with_port, runtime};
use tokio::signal;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    runtime::run(&[with_port(777)], signal::ctrl_c());
}
