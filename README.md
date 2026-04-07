# GX Exchange Rust SDK

Official Rust SDK for the GX Exchange API. Provides a performant, type-safe interface for programmatic trading and market data access on the GX Chain network.

## What It Does

The Rust SDK gives low-latency trading systems native access to GX Chain's matching engine. It includes typed request/response structures, EIP-712 signing, and both REST and WebSocket clients for order execution and real-time data streaming.

## Quick Start

```bash
cargo add gx_rust_sdk
```

## Usage

### Initialize and Place an Order

```rust
use gx_rust_sdk::{ExchangeClient, InfoClient, MAINNET_API_URL};

let info = InfoClient::new(None, Some(MAINNET_API_URL)).await?;
let exchange = ExchangeClient::new(None, wallet, Some(MAINNET_API_URL), None, None).await?;

let order = exchange.order("ETH", true, 0.2, 1100.0, OrderType::Limit(Tif::Gtc), None).await?;
println!("{:?}", order);
```

### Fetch Market Metadata

```rust
let meta = info.meta().await?;
for asset in meta.universe {
    println!("{}: {} decimals", asset.name, asset.sz_decimals);
}
```

### Subscribe to Order Book Updates

```rust
let (sender, receiver) = tokio::sync::mpsc::channel(100);
info.subscribe(Subscription::L2Book { coin: "ETH".into() }, sender).await?;
while let Some(msg) = receiver.recv().await {
    println!("{:?}", msg);
}
```

## Configuration

| Parameter | Description |
|---|---|
| `MAINNET_API_URL` | Production API endpoint |
| `TESTNET_API_URL` | Testnet API endpoint |
| `LOCAL_API_URL` | Local node API endpoint |

## Requirements

- Rust toolchain (stable)

## Documentation

- [Rust SDK Reference](https://docs.gx.exchange/for-developers/api/rust-sdk)
- [API Reference](https://docs.gx.exchange/for-developers)
- [Trading Guide](https://docs.gx.exchange/trading)

## Links

- [GX Exchange GitHub](https://github.com/GX-EXCHANGE)
- [GX Exchange](https://gx.exchange)

## License

MIT
