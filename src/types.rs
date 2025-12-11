use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    
    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: String,
}