#[macro_use]
extern crate serde;

use serde_json::{from_str, self};

pub mod models{
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Account{

        #[serde(rename = "ID")]
        pub id: u64,

        #[serde(rename = "NAME")]
        pub name: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Order{

        #[serde(rename = "i")]
        pub id: u64,

        #[serde(rename = "s")]
        pub symbol: String,
    }

    // enum表达式 https://serde.rs/enum-representations.html
    #[derive(Debug, Deserialize)]
    #[serde(tag = "e")]
    pub enum Event {
        #[serde(alias = "account")]
        Account(Box<Account>),

        #[serde(alias = "order")]
        Order(Box<Order>),
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "e", content="data")]
    pub enum EventInside {
        #[serde(alias = "account")]
        Account(Box<Account>),

        #[serde(alias = "order")]
        Order(Box<Order>),
    }

}

fn main() {
    let account: models::Account = from_str(r#"{"ID": 123, "NAME": "Dean"}"#).unwrap();
    println!("{account:?}");
    let json_str = serde_json::to_string(&account).unwrap();
    println!("{json_str}");

    let events: Vec<models::Event> = vec![
        from_str(r#"{"e":"account", "ID": 123, "NAME": "Dean"}"#).unwrap(),
        from_str(r#"{"e":"order", "i": 233, "s": "BTC"}"#).unwrap(),
    ];
    for event in events{
        match event{
            models::Event::Account(account)=> {println!("account: {account:?}");},
            models::Event::Order(order)=> {println!("order: {order:?}");},
        }
    }

    let events: Vec<models::EventInside> = vec![
        from_str(r#"{"e":"account", "data": {"ID": 123, "NAME": "Dean"}}"#).unwrap(),
        from_str(r#"{"e":"order", "data": {"i": 233, "s": "BTC"}}"#).unwrap(),
    ];
    for event in events{
        match event{
            models::EventInside::Account(account)=> {println!("account: {account:?}");},
            models::EventInside::Order(order)=> {println!("order: {order:?}");},
        }
    }
}
