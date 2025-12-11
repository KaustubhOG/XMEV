use crate::jupiter_client::JupiterClient;

pub async fn run_bot() {
    println!("Bot is running");
    let client = JupiterClient::new();
    println!("{}", client.base_url);
    
}
