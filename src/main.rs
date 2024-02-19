pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

use infrastructure::main_cli::run_cli;

#[tokio::main]
async fn main() {
    run_cli();
}
