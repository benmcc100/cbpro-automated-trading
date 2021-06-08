use coinbase_pro_api_rust::client::AuthorizedClient;
use std::thread;
use std::time::Duration;
use serde_json::Value;
use std::collections::HashMap;
use rust_decimal::prelude::{Decimal, FromStr};

pub struct TradingData<T> {
    pub products: HashMap<String, ProductData>,
    pub user_data: T,
}

pub struct ProductData {
    pub product_id: String,
    pub price: Decimal,
    pub best_bid: Decimal,
    pub best_ask: Decimal,
    pub high_24h: Decimal,
    pub low_24h: Decimal,
    pub volume_24h: Decimal,
    pub volume_30d: Decimal,
    pub last_trade: Trade,
}

pub struct Trade {
    pub trade_id: u64,
    pub price: Decimal,
    pub side: String,
    pub size: Decimal,
}

pub struct Strategy<T> {
    name: String,
    client: AuthorizedClient,
    products: Vec<String>,
    data: TradingData<T>, // maybe something like this??
    strategy: fn(&AuthorizedClient, &mut TradingData<T>),
}

impl<T> Strategy<T> {
    pub fn new(
        name: &str, 
        client: AuthorizedClient,
        products: Vec<&str>, 
        strategy: fn(&AuthorizedClient, &mut TradingData<T>),
        user_data: T,
    ) -> Self {
        Strategy {
            name: name.to_string(),
            client,
            products: products.iter().map(|s| s.to_string()).collect(),
            data: TradingData{ products: HashMap::new(), user_data },
            strategy,
        }
    }

    pub fn subscribe_to_ticker_data(&mut self) {
        &self.client.connect_socket();
        &self.client.subscribe(self.products.iter().map(|s| &s[..]).collect(), vec!["ticker"]);
    }

    pub fn run(&mut self) {
        
        loop {
            if let Some(ticker_data) = self.client.read_from_ws().unwrap().into_text().ok() {
                let ticker_data: Value = serde_json::from_str(&ticker_data).unwrap();
                // println!("{}", ticker_data);
                match &ticker_data["product_id"] {
                    serde_json::Value::Null => continue,
                    serde_json::Value::String(s) => {
                        self.update_product(s.to_string(), &ticker_data);
                    },
                    _ => continue,
                }
            
                (self.strategy)(&self.client, &mut self.data)
                // let action_block = async {
                //     (self.strategy)(&self.client, &self.data)
                // };
                // rt.block_on(action_block);
                
            }
            // may or may not be necessary
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn update_product(&mut self, product: String, data: &Value) {
        let new_item = ProductData {
            product_id: product.to_string(),
            price: Decimal::from_str(data["price"].as_str().unwrap()).unwrap(), 
            best_bid: Decimal::from_str(data["best_bid"].as_str().unwrap()).unwrap(),
            best_ask: Decimal::from_str(data["best_ask"].as_str().unwrap()).unwrap(),
            high_24h: Decimal::from_str(data["high_24h"].as_str().unwrap()).unwrap(),
            low_24h: Decimal::from_str(data["low_24h"].as_str().unwrap()).unwrap(),
            volume_24h: Decimal::from_str(data["volume_24h"].as_str().unwrap()).unwrap(),
            volume_30d: Decimal::from_str(data["volume_30d"].as_str().unwrap()).unwrap(),
            last_trade: Trade {
                trade_id: data["trade_id"].as_u64().unwrap(),
                price: Decimal::from_str(data["price"].as_str().unwrap()).unwrap(),
                side: data["side"].as_str().unwrap().to_string(),
                size: Decimal::from_str(data["last_size"].as_str().unwrap()).unwrap(),
            }
        };
        self.data.products.insert(product, new_item);
    }
}
