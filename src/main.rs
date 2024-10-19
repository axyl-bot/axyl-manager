mod auth;
mod bot;
mod testing;
mod user_info;

use crate::bot::start_bot;
use crate::testing::run_test_server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "test_server" {
        run_test_server().await;
    } else if let Err(err) = start_bot().await {
        eprintln!("Error: {}", err);
    }
}
