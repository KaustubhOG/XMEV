mod utils;
use crate::utils::run_bot;

#[tokio::main]
async fn main() {
    println!("everything is working ");
    run_bot().await;
}
