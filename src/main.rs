mod engine;
use engine::*;
use std::time::Instant;
use rand::Rng;
use ordered_float::OrderedFloat;

fn main() {
    let mut book = OrderBook::new();
    let mut rng = rand::rng();
    let total_orders = 1_000_000;

    // Generate random orders
    println!("Generating random data...");
    let mut orders = Vec::with_capacity(total_orders);
    for i in 0..total_orders {
        let price = rng.random_range(90.0..110.0);
        let quantity = rng.random_range(1..100);
        let side = if rng.random_bool(0.5) { Side::Buy } else { Side::Sell };

        orders.push(Order {
            _id: i as u64,
            price,
            quantity,
            side,
        });
    }
    println!("Generated orders...\n");

    // Simulating the market
    println!("Starting the simulation...");
    let start = Instant::now();
    for order in orders {
        book.add_order(order);
    }
    println!("End of simulation....\n");

    // Benchmarks
    let duration = start.elapsed();
    let seconds = duration.as_secs_f64();
    let throughput = total_orders as f64 / seconds;
    let latency_per_order = (seconds * 1_000_000_000.0) / total_orders as f64;
    println!("Simulation finished in: {:?}", duration);
    println!("Throughput: {:.2} seconds", throughput);
    println!("Latency per order: {:.2} nanoseconds", latency_per_order);

    // OUTPUT:-
    // Simulation finished in: 90.205291ms
    // Throughput: 11085824.22 seconds
    // Latency per order: 90.21 nanoseconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_match() {
        let mut book = OrderBook::new();
        // Sell 100 @ 150
        book.add_order(Order { _id: 1, price: 150.0, quantity: 100, side: Side::Sell });

        // Buy 50 @ 150 (Should match)
        book.add_order(Order { _id: 2, price: 150.0, quantity: 50, side: Side::Buy });

        // Verify state: The Sell order should have 50 left
        let best_ask = book.asks.values().next().unwrap().front().unwrap();
        assert_eq!(best_ask.quantity, 50);
    }

    #[test]
    fn test_price_priority() {
        let mut book = OrderBook::new();
        // Sell 100 @ 150
        book.add_order(Order { _id: 1, price: 150.0, quantity: 100, side: Side::Sell });
        // Sell 100 @ 140 (Better price!)
        book.add_order(Order { _id: 2, price: 140.0, quantity: 100, side: Side::Sell });

        // Buy 100 @ 150. Should match the 140 sell first because it's cheaper.
        book.add_order(Order { _id: 3, price: 150.0, quantity: 100, side: Side::Buy });

        // The 140 ask should be gone. The 150 ask should remain.
        assert!(book.asks.get(&OrderedFloat(140.0)).is_none());
        assert!(book.asks.get(&OrderedFloat(150.0)).is_some());
    }
}
