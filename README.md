# Solana MEV Arbitrage Bot
## Overview
A high-frequency trading bot designed to identify and exploit arbitrage opportunities across various decentralized exchanges (DEXs) on the Solana blockchain.

## Features

- **Multi-DEX Support**: Works with Raydium, Orca Whirlpools, and Meteora DEXs
- **Real-time Pool Monitoring**: Continuously scans for new liquidity pools
- **Advanced Arbitrage Detection**: Identifies profitable 1-hop and 2-hop arbitrage paths
- **Simulation Engine**: Tests potential trades before execution
- **Optimized Execution**: Prioritizes the most profitable opportunities
- **Performance Tracking**: Records all arbitrage attempts and results

## Supported DEXs

- Raydium (CLMM and standard pools)
- Orca Whirlpools
- Meteora

## Code Structure
```
src/
├── arbitrage/
│ ├── calc_arb.rs # Arbitrage calculation logic
│ ├── simulate.rs # Trade simulation
│ ├── streams.rs # Real-time data streams
│ └── types.rs # Data structures
├── markets/
│ ├── meteora.rs # Meteora DEX integration
│ ├── orca_whirpools.rs # Orca integration
│ ├── raydium.rs # Raydium integration
│ └── types.rs # Market data structures
└── common/ # Shared utilities and constants
```

## Key Components

### Pool Discovery
```rust
pub async fn get_fresh_pools(tokens: Vec<TokenInArb>) -> HashMap<String, Market> {
    // Scans supported DEXs for new pools containing specified tokens
    // Implements rate limiting between requests
}