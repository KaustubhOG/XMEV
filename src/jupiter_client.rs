use crate::types::QuoteResponse;
use anyhow::Result;

pub struct JupiterClient {
    pub http_client: reqwest::Client,
    pub base_url: String,
}

impl JupiterClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

        Self {
            http_client: client,
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

        let res = self
            .http_client
            .get(&url)
            .send()
            .await?
            .json::<QuoteResponse>()
            .await?;

        Ok(res)
    }
}
