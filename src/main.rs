use std::collections::{BTreeMap, VecDeque};
use ordered_float::OrderedFloat;
use rand::Rng;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
struct Order {
    _id: u64,
    price: f64,
    quantity: u64,
    side: Side,
}

struct OrderBook {
    bids: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,
    asks: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,
}

impl OrderBook {
    fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn add_order(&mut self, mut order: Order) {
        match order.side {
            Side::Buy => self.match_bid(&mut order),
            Side::Sell => self.match_ask(&mut order),
        }
    }

    fn match_bid(&mut self, order: &mut Order) {
        while order.quantity > 0 {
            // Use first_entry() because Bids are sorted Low -> High by default.
            // The "Best" ask is the LOWEST price, which is at the start of the map.
            if let Some(mut entry) = self.asks.first_entry() {
                let best_ask_price = *entry.key();
                let ask_queue = entry.get_mut();

                // If sellers are too expensive, break
                if best_ask_price.into_inner() > order.price {
                    break;
                }

                // Execute the trade
                let best_ask_order = ask_queue.front_mut().unwrap();
                let trade_qty = order.quantity.min(best_ask_order.quantity);
                // println!("Trade! Price: {}, Qty: {}", best_ask_price, trade_qty);

                // Update the quantities as per the trade quantity
                order.quantity -= trade_qty;
                best_ask_order.quantity -= trade_qty;

                // Remove completed orders from queue
                if best_ask_order.quantity == 0 {
                    ask_queue.pop_front();
                }

                // Cleanup empty price levels
                if ask_queue.is_empty() {
                    entry.remove();
                }
            } else {
                break; // No sellers
            }
        }

        // If not fully filled, rest on the book
        if order.quantity > 0 {
            self.bids.entry(OrderedFloat(order.price))
                .or_default()
                .push_back(order.clone());
            // println!("Buy Order rested: {} @ {}", order.quantity, order.price);
        }
    }

    fn match_ask(&mut self, order: &mut Order) {
        while order.quantity > 0 {
            // Use last_entry() because Bids are sorted Low -> High by default.
            // The "Best" bid is the HIGHEST price, which is at the end of the map.
            if let Some(mut entry) = self.bids.last_entry() {
                let best_bid_price = *entry.key();
                let bid_queue = entry.get_mut();

                // If buyers are too cheap, break
                if best_bid_price.into_inner() < order.price {
                    break;
                }

                // Execute the trade
                let best_bid_order = bid_queue.front_mut().unwrap();
                let trade_qty = order.quantity.min(best_bid_order.quantity);
                // println!("Trade Executed! Price: {}, Qty: {}", best_bid_price, trade_qty);

                // Update the quantities as per the trade quantity
                order.quantity -= trade_qty;
                best_bid_order.quantity -= trade_qty;

                // Remove completed orders from queue
                if best_bid_order.quantity == 0 {
                    bid_queue.pop_front();
                }

                // Cleanup empty price levels
                if bid_queue.is_empty() {
                    entry.remove();
                }
            } else {
                break; // No buyers
            }
        }

        // If not fully filled, rest on the book
        if order.quantity > 0 {
            self.asks.entry(OrderedFloat(order.price))
                .or_default()
                .push_back(order.clone());
            // println!("Sell Order rested: {} @ {}", order.quantity, order.price);
        }
    }
}

fn main() {
    let mut book = OrderBook::new();

    // Selling 100 shares at $150
    let sell_order = Order { _id: 1, price: 150.0, quantity: 100, side: Side::Sell };
    book.add_order(sell_order);
    println!("Ask placed. Book state updated.");

    // Buying 50 shares at $150
    let buy_order = Order { _id: 2, price: 150.0, quantity: 50, side: Side::Buy };
    println!("Placing Buy Order...");
    book.add_order(buy_order);

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
