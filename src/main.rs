mod app;
mod cli;
mod core;
mod http;
#[cfg(test)]
mod test_env;
mod tooling;

pub use crate::core::config;
pub use crate::core::managed_binaries;
pub use crate::core::self_update;
pub use crate::http::client;
pub use crate::tooling::tool_executor;

#[tokio::main]
async fn main() {
    if let Err(error) = app::runner::run().await {
        eprintln!("Error: {error:#}");
        std::process::exit(1);
    }
}
