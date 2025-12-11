mod utils;
mod jupiter_client;
mod jito_client;
mod types;

#[tokio::main]
async fn main() {
    println!("everything is working");

    if let Err(e) = utils::run_bot().await {
        println!("Error: {:?}", e);
    }
}
