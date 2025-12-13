use anyhow::Result;
use base64;
use solana_sdk::transaction::VersionedTransaction;

use crate::jupiter_client::JupiterClient;
use crate::jito_client::JitoClient;

pub async fn run_bot() -> Result<()> {
    println!("Bot is running...");

    let client = JupiterClient::new();
    let jito = JitoClient::new();

    let user_pubkey = "AR8rRkMAcYRpeFZLJeTz5vbGMFy5yrMqNEoEewoGW7hR";

    //GET QUOTE
    let quote = client.get_quote(
        "So11111111111111111111111111111111111111112",
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        1_000_000_000,
    ).await?;
    println!("Quote: {:?}", quote);

    //GET SWAP TRANSACTION
    let swap_tx = client.get_swap_tx(quote, user_pubkey).await?;
    println!("Swap transaction received!");

    //DECODE & DESERIALIZE
    let bytes = base64::decode(&swap_tx.swap_transaction)?;
    let tx: VersionedTransaction = bincode::deserialize(&bytes)?;
    println!("Decoded VersionedTransaction!");

    //PRINT DETAILS
    println!("Transaction has {} instructions", tx.message.instructions().len());

    //SEND TO JITO
    let jito_response = jito.send_bundle(swap_tx.swap_transaction).await?;
    println!("Jito Bundle Response: {}", jito_response);

    Ok(())
}
