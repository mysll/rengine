use re_core::runtime;
use tokio::signal;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    runtime::run(777, signal::ctrl_c());
}
