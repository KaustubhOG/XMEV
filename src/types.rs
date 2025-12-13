use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuoteResponse {
    #[serde(rename = "inputMint")]
    pub input_mint: String,

    #[serde(rename = "inAmount")]
    pub in_amount: String,

    #[serde(rename = "outputMint")]
    pub output_mint: String,

    #[serde(rename = "outAmount")]
    pub out_amount: String,

    #[serde(rename = "otherAmountThreshold")]
    pub other_amount_threshold: String,

    #[serde(rename = "swapMode")]
    pub swap_mode: String,

    #[serde(rename = "slippageBps")]
    pub slippage_bps: u16,

    #[serde(rename = "platformFee")]
    pub platform_fee: Option<serde_json::Value>,

    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: String,

    #[serde(rename = "routePlan")]
    pub route_plan: Vec<serde_json::Value>,

    #[serde(rename = "contextSlot")]
    pub context_slot: u64,

    #[serde(rename = "timeTaken")]
    pub time_taken: f64,

    #[serde(rename = "swapUsdValue")]
    pub swap_usd_value: String,

    #[serde(rename = "simplerRouteUsed")]
    pub simpler_route_used: bool,

    #[serde(rename = "mostReliableAmmsQuoteReport")]
    pub most_reliable_amms_quote_report: Option<serde_json::Value>,

    #[serde(rename = "useIncurredSlippageForQuoting")]
    pub use_incurred_slippage_for_quoting: Option<serde_json::Value>,

    #[serde(rename = "otherRoutePlans")]
    pub other_route_plans: Option<serde_json::Value>,

    #[serde(rename = "loadedLongtailToken")]
    pub loaded_longtail_token: bool,

    #[serde(rename = "instructionVersion")]
    pub instruction_version: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct SwapResponse {
    #[serde(rename = "swapTransaction")]
    pub swap_transaction: String,
}

#[derive(Debug, Serialize)]
pub struct SwapRequest {
    #[serde(rename = "quoteResponse")]
    pub quote_response: QuoteResponse,

    #[serde(rename = "userPublicKey")]
    pub user_public_key: String,

    #[serde(rename = "wrapAndUnwrapSol")]
    pub wrap_and_unwrap_sol: bool,
}
