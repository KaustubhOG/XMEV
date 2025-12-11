mod utils;
use crate::utils::run_bot;
mod jupiter_client;


#[tokio::main]
async fn main() {
    println!("everything is working ");
    run_bot().await;
}
