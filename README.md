# lob-rs: High-Performance Rust-based Limit Order Book (LOB)

> **"Speed vs. Safety" is no longer a debate. Both can exist simultaneously.**

RLOB is a deterministic, in-memory Limit Order Book (LOB) matching engine written in **Rust**. It is designed to benchmark the zero-cost abstractions of Rust against traditional C++ HFT architectures, proving that we can achieve nanosecond-level latency without compromising on memory safety.

## Performance Benchmarks

Benchmarks run on commodity hardware (single-threaded execution).

| Metric              | Result                        |
|:--------------------|:------------------------------|
| **Throughput**      | **~11,000,000 orders/sec**    |
| **Average Latency** | **~90 nanoseconds** per order |
| **Memory Safety**   | **Guaranteed with Rust**      |

*Benchmark Methodology: Sequentially processing 1,000,000 randomized orders (Ask/Bid mix) with full matching logic and book updates.*

## Architecture

The engine implements a standard **Price-Time Priority** matching algorithm using standard library collections.

### Core Data Structures
```rust
struct OrderBook {
    bids: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,
    asks: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,
}
````

* **`BTreeMap`**: Used for price levels to maintain sorted order ($O(\log N)$ insertion/removal). Chosen over `HashMap` to support efficient iteration of best prices.
* **`VecDeque`**: Used for the order queue at each price level to enforce strict **FIFO** (Time Priority) ordering.
* **`OrderedFloat`**: Handles floating-point constraints (NaN safety) for price keys.

### Matching Logic

1.  **Incoming Order:** The engine identifies the side (Buy/Sell).
2.  **Crossing the Book:** It aggressively matches against the opposite side's best price level.
    * *Partial Fills:* If the best resting order cannot fill the incoming order, it is consumed, and the engine moves to the next order in the queue.
    * *Price Aggression:* Matching stops immediately if the best available price exceeds the limit price.
3.  **Resting:** Any remaining quantity is placed into the book at its limit price.

## Usage

### Prerequisites

* Rust (stable)
* Cargo

### Running the Engine

```bash
# Clone the repository
git clone https://github.com/Aditya-A-Thakkar/lob-rs.git
cd lob-rs

# Run the benchmark
cargo run --release
```

## Further plans

* **Lock-Free Concurrency:** Implement `LMAX Disruptor` pattern for multi-threaded input.
* **FFI Bindings:** Expose a C-API to allow integration with C++ trading gateways.
* **TCP Gateway:** Wrap the engine in an async `tokio` TCP server for remote order entry.

## 📜 License
MIT

## Author
[![Aditya Thakkar](https://img.shields.io/badge/Aditya%20Thakkar-blue?&style=for-the-badge)](https://github.com/Aditya-A-Thakkar)
