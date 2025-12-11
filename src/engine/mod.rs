use std::collections::{BTreeMap, VecDeque};
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub _id: u64,
    pub price: f64,
    pub quantity: u64,
    pub side: Side,
}

pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,
    pub asks: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, mut order: Order) {
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