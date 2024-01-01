// use crate::order_engines::{create_buy_order, create_sell_order};

// use std::fmt::Debug;

use axum::{routing::post, Router};
use serde::Deserialize;

#[derive(Clone)]
pub enum INSTRUMENT {
    GOOG,
    APPL,
    NVD,
    TSL,
}

#[derive(Clone, Deserialize)]
pub enum BidOrAsk {
    BID,
    ASK,
}

// fn print_vec<T: Debug>(v: &Vec<T>) {
//     println!("####### DEBUG #######");

//     for i in v.iter() {
//         println!("{:?}", i);
//     }
// }

#[derive(Deserialize)]
struct OrderBody {
    price: f64,
    quantity: u16,
    order_type: BidOrAsk,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/order", post(order_handler::new_order));

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub mod order_handler {
    use axum::Json;

    use crate::{order_manager, BidOrAsk, Order, OrderBody, OrderBook};

    pub async fn new_order(Json(order_data): Json<OrderBody>) {
        // format!("Added item: {}", item.title)
        let mut order_book = OrderBook::new();
        order_manager::create_order(
            Order::new(
                order_data.price,
                order_data.quantity,
                order_data.order_type.clone(),
            ),
            order_data.order_type.clone(),
            &mut order_book,
        );
    }
}

// #[derive(Debug)]
// fn main() {
//     let mut order_book = OrderBook::new();

//     order_manager::create_order(
//         Order::new(10.1, 100, BidOrAsk::ASK),
//         BidOrAsk::ASK,
//         &mut order_book,
//     );
//     order_manager::create_order(
//         Order::new(10.0, 150, BidOrAsk::ASK),
//         BidOrAsk::ASK,
//         &mut order_book,
//     );
//     order_manager::create_order(
//         Order::new(10.2, 10, BidOrAsk::ASK),
//         BidOrAsk::ASK,
//         &mut order_book,
//     );
//     order_manager::create_order(
//         Order::new(9.98, 100, BidOrAsk::ASK),
//         BidOrAsk::ASK,
//         &mut order_book,
//     );
//     order_manager::create_order(
//         Order::new(9.8, 200, BidOrAsk::BID),
//         BidOrAsk::BID,
//         &mut order_book,
//     );
//     order_manager::create_order(
//         Order::new(10.1, 10, BidOrAsk::BID),
//         BidOrAsk::BID,
//         &mut order_book,
//     );
//     order_manager::create_order(
//         Order::new(10.5, 20, BidOrAsk::ASK),
//         BidOrAsk::ASK,
//         &mut order_book,
//     );
//     order_manager::create_order(
//         Order::new(9.5, 100, BidOrAsk::BID),
//         BidOrAsk::BID,
//         &mut order_book,
//     );
// }

#[derive(Clone)]
pub struct Order {
    // user: u32,
    // instrument: INSTRUMENT,
    price: f64,
    quantity: u16,
}

impl Order {
    fn new(price: f64, quantity: u16, order_type: BidOrAsk) -> Order {
        Order {
            // user: 123,
            // instrument: INSTRUMENT::APPL,
            price,
            quantity,
        }
    }
}

// impl Ord for Order {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }

pub struct OrderBook {
    // instrument: String,
    bids: Vec<Order>,
    asks: Vec<Order>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bids: Vec::<Order>::new(),
            asks: Vec::<Order>::new(),
        }
    }

    pub fn insert_order(&mut self, order: Order, order_type: BidOrAsk) {
        match order_type {
            BidOrAsk::ASK => {
                println!("--------------------------------------");
                self.asks.push(order.clone());
                self.asks.sort_by(|a, b| b.price.total_cmp(&a.price));
                // for i in self.asks.iter() {
                //     println!("{}", i.price);
                // }

                let is_match = order_engine::match_orders(&order, &mut self.bids);

                println!("####### DEBUG MATCH RESULT #######");
                println!("{is_match}");
            }
            BidOrAsk::BID => {
                self.bids.push(order.clone());
                self.bids.sort_by(|a, b| a.price.total_cmp(&b.price));

                let is_match = order_engine::match_orders(&order, &mut self.bids);

                println!("####### DEBUG MATCH RESULT #######");
                println!("{is_match}");
            }
        }
    }
}

pub mod order_manager {
    use crate::{BidOrAsk, Order, OrderBook};

    pub fn create_order(order: Order, order_type: BidOrAsk, order_book: &mut OrderBook) {
        order_book.insert_order(order, order_type)
    }
}

// pub struct OrderEngine {}

pub mod order_engine {
    // pub fn new() -> OrderEngine {
    //     return OrderEngine {
    //         sell_order_list: Vec::new(),
    //         buy_order_list: Vec::new(),
    //     };
    // }

    use crate::Order;

    pub fn match_orders(
        order: &Order,
        // order_type: BidOrAsk,
        candidates: &mut Vec<Order>,
    ) -> bool {
        let order_price = order.price;
        let mut remaining_quantity = order.quantity;

        for c in candidates.iter_mut() {
            if remaining_quantity == 0 || c.price > order_price {
                break;
            }

            if remaining_quantity <= c.quantity {
                c.quantity -= remaining_quantity;
                remaining_quantity = 0;
                break;
            } else {
                let current_candidate_quanity = c.quantity.clone();
                println!("{}, {}", c.quantity, remaining_quantity);
                c.quantity -= remaining_quantity;
                remaining_quantity -= current_candidate_quanity;
            }
        }

        println!(
            "!!!!!!!!!!!!!!!!!! Candidates len before: {}",
            candidates.len()
        );
        candidates.retain(|c| c.quantity > 0);
        println!(
            "!!!!!!!!!!!!!!!!!! Candidates len after: {}",
            candidates.len()
        );

        remaining_quantity == 0
    }
}
