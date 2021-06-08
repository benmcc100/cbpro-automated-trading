use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};
///
/// Order types
///
pub enum Order {
    MarketOrder,
    LimitOrder,
}

///
/// Market order
///
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketOrder {
    r#type: String,
    size: Decimal,
    side: String,
    product_id: String,
}

impl MarketOrder {
    pub fn new(r#type: String, size: Decimal, side: String, product_id: String) -> Self {
        MarketOrder {
            r#type,
            size,
            side,
            product_id,
        }
    }
}

///
/// Limit order
///
#[derive(Serialize, Deserialize, Debug)]
pub struct LimitOrder {
    r#type: String,
    price: Decimal,
    size: Decimal,
    side: String,
    product_id: String,
}

impl LimitOrder {
    pub fn new(
        r#type: String,
        price: Decimal,
        size: Decimal,
        side: String,
        product_id: String,
    ) -> Self {
        LimitOrder {
            r#type,
            price,
            size,
            side,
            product_id,
        }
    }
}

///
/// Order response
///
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderResponse {
    id: String,
    price: Option<Decimal>,
    size: String,
    product_id: String,
    side: String,
    stp: String,
    funds: String,
    r#type: String,
    time_in_force: Option<String>,
    post_only: bool,
    created_at: String,
    fill_fees: String,
    filled_size: String,
    executed_value: String,
    status: String,
    settled: bool,
}

///
/// Open orders
///
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenOrder {
    id: String,
    price: Option<String>,
    size: Option<String>,
    product_id: String,
    profile_id: String,
    side: String,
    funds: Option<String>,
    specified_funds: Option<String>,
    stp: Option<String>,
    r#type: String,
    time_in_force: Option<String>,
    post_only: bool,
    created_at: String,
    done_at: Option<String>,
    done_reason: Option<String>,
    fill_fees: String,
    filled_size: String,
    executed_value: String,
    status: String,
    settled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListOrder {
    status: String,
}

impl ListOrder {
    pub fn new(status: &str) -> Self {
        ListOrder {
            status: status.to_string(),
        }
    }
}
