use crate::jupiter_client::JupiterClient;
use anyhow::Result;

pub async fn run_bot() -> Result<()> {
    println!("Bot is running");

    let client = JupiterClient::new();

    let quote = client
        .get_quote(
            "So11111111111111111111111111111111111111112",  // SOL
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC (not the broken token!)
            1_000_000_000,                                  // 1 SOL
        )
        .await?;
    println!("{:?}", quote);

    Ok(())
}
