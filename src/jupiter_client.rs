pub struct JupiterClient {
    pub http_client: reqwest::Client,
    pub base_url: String,
}

impl JupiterClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

        Self {
            http_client: client,
            base_url: "https://quote-api.jup.ag/v6".to_string(),
        }
    }
}
