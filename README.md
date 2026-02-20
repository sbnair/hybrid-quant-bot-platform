# Hybrid Quant Bot Platform

A production‑ready reference architecture for building quantitative trading bots that operate across both Centralized (CEX) and Decentralized (DeFi) exchanges. This project demonstrates how to combine Python, Rust, and C++ to implement volatility strategies such as variance swap replication, dispersion trading, and gamma scalping.

## Features

- **Rust pricing library** – Computes fair variance swap strikes from option chains (exposed via C ABI).
- **C++ order router** – Simulates low‑latency order sending to a CEX (exposed via C ABI).
- **Python orchestration** – Loads both libraries, fetches mock market data, detects arbitrage, and executes orders.
- **Async Rust risk monitor** – Real‑time gamma exposure and DeFi pool depth monitoring (standalone binary).

## Architecture

+-------------------+ +-------------------+ +-------------------+
| Python Layer | <--> | Rust Layer | <--> | C++ Layer |
| (Research/Backtest)| | (Pricing/Risk/ | | (CEX Execution) |
| | | Data Aggregation) | | |
+-------------------+ +-------------------+ +-------------------+
^ ^ ^
| (mock data) | (mock data) | (mock calls)
v v v
[Mock Market Data] [Mock Feeds] [Mock Exchange]


## Prerequisites

- Rust (stable) – https://rustup.rs/
- C++ compiler with C++17 support (gcc/clang/msvc)
- CMake (optional, a Makefile is also provided)
- Python 3.8+ with `numpy` and `ctypes` (standard library)

## Building

### 1. Rust Pricing Library

```bash
cd rust
cargo build --release
# The shared library will be at target/release/libpricing.so (or .dylib/.dll)

cd cpp
mkdir build && cd build
cmake ..
make
# The shared library will be at build/liborderrouter.so

cd cpp
make

cd python
pip4 install numpy

cd python
python3 main.py

cd rust
cargo run --bin risk_monitor
 ```
