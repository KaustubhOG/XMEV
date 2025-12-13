# XMEV — Solana MEV Bot (Jupiter + Jito)

Minimal Solana MEV bot in **Rust** demonstrating an end-to-end pipeline with **Jupiter** (quote & swap) and **Jito** (bundle submission).  
Focus: infra, transaction flow, and correctness — not a production trading strategy.

## Features
- Fetches live swap quotes from Jupiter  
- Requests swap transactions (base64) from Jupiter  
- Decodes & deserializes `VersionedTransaction` (Solana SDK)  
- Inspects instructions & accounts  
- Submits transaction bundles to Jito relayer

## Quick start
```bash
# build & run
cargo run
You’ll see quote output, decoded transaction info, and the Jito relayer response (public endpoint may be rate-limited).

Project layout
bash
Copy code
src/
 ├─ main.rs
 ├─ utils/            # run_bot + testing flow
 ├─ jupiter_client.rs # Jupiter API client
 ├─ jito_client.rs    # Jito bundle submission
 └─ types.rs          # request/response models
Notes
Transactions returned by Jupiter are unsigned. Signing (private-key handling) is intentionally out of scope.

Public Jito endpoints are often rate-limited; unsigned bundles may be rejected or deprioritized.

Tech
Rust · Tokio · reqwest · serde · solana-sdk · base64 · bincode
