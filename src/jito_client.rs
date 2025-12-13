use anyhow::Result;
use serde::Serialize;

pub struct JitoClient {
    pub http_client: reqwest::Client,
    pub base_url: String,
}

impl JitoClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: "https://ny.mainnet.block-engine.jito.wtf/api/v1/bundles".to_string(),
        }
    }

    pub async fn send_bundle(&self, tx_base64: String) -> Result<String> {
        #[derive(Serialize)]
        struct BundleRequest {
            transactions: Vec<String>,
        }

        let body = BundleRequest {
            transactions: vec![tx_base64],
        };

        let resp = self.http_client
            .post(&self.base_url)
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }
}
