use rust_decimal::prelude::*;

struct Ask {
    pub price: f64,
    pub amount: f64,
}

struct Depth {
    pub asks: Vec<Ask>,
    pub depth: u64,
    
}

fn main() {
    let depth= Depth {
        asks: vec![
            Ask{price:1.11, amount:1.1},
            Ask{price:1.23, amount:1.0},
            Ask{price:1.37, amount:1.1},
            Ask{price:1.41, amount:1.0},
        ],
        depth: 3,
    };

    let mut ask_avg_2: f64 = depth.asks.iter().take(3).map(|a|a.price).sum();
    ask_avg_2 /= 2.0;
    println!("ask_avg:{}", ask_avg_2);

    let sell_price = Decimal::from_f64(ask_avg_2 * 1.2).unwrap();
    let sell_price_f64 = sell_price.round_dp(2).to_f64().unwrap();
    println!("sell_price_f64:{}", sell_price_f64);
}
