use crate::types::{QuoteResponse, SwapRequest, SwapResponse};
use anyhow::Result;

pub struct JupiterClient {
    pub http_client: reqwest::Client,
    pub base_url: String,
}

impl JupiterClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: "https://lite-api.jup.ag/swap/v1".to_string(),
        }
    }

    pub async fn get_quote(
        &self,
        input_mint: &str,
        output_mint: &str,
        amount: u64,
    ) -> Result<QuoteResponse> {
        let url = format!(
            "{}/quote?inputMint={}&outputMint={}&amount={}",
            self.base_url, input_mint, output_mint, amount
        );

        Ok(self.http_client.get(&url)
            .send().await?
            .json::<QuoteResponse>().await?)
    }

    pub async fn get_swap_tx(
        &self,
        quote: QuoteResponse,
        user_pubkey: &str,
    ) -> Result<SwapResponse> {

        let url = format!("{}/swap", self.base_url);

        let swap_request = SwapRequest {
            quote_response: quote,
            user_public_key: user_pubkey.to_string(),
            wrap_and_unwrap_sol: true,
        };

        Ok(self.http_client
            .post(&url)
            .json(&swap_request)
            .send()
            .await?
            .json::<SwapResponse>()
            .await?)
    }
}
