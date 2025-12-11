use std::collections::{BTreeMap, VecDeque};
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
struct Order {
    id: u64,
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
            // Peek at cheapest seller
            if let Some((best_ask_price, ask_queue)) = self.asks.first_key_value() {
                // If sellers are too expensive, break
                if best_ask_price.into_inner() > order.price {
                    break;
                }

                let mut best_ask = ask_queue[0].quantity;
                let trade_qty = order.quantity.min(best_ask);

                println!("Trade! Price: {}, Qty: {}", best_ask_price, trade_qty);

                order.quantity -= trade_qty;
                best_ask -= trade_qty;

                if best_ask == 0 {
                    self.asks.get_mut(best_ask_price).pop_front();
                }

                // Cleanup empty price levels
                if ask_queue.is_empty() {
                    let price_copy = *best_ask_price;
                    self.asks.remove(&price_copy);
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
        }
    }

    fn match_ask(&mut self, order: &mut Order) {
        // TODO: complete this function
        // Placeholder for now:
        if order.quantity > 0 {
            self.asks.entry(OrderedFloat(order.price))
                .or_default()
                .push_back(order.clone());
        }
    }
}

fn main() {
    let mut book = OrderBook::new();

    // Selling 100 shares at $150
    let sell_order = Order { id: 1, price: 150.0, quantity: 100, side: Side::Sell };
    book.add_order(sell_order);
    println!("Ask placed. Book state updated.");

    // Buying 50 shares at $150
    let buy_order = Order { id: 2, price: 150.0, quantity: 50, side: Side::Buy };
    println!("Placing Buy Order...");
    book.add_order(buy_order);
}
