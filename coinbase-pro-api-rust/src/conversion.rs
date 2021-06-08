use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

///
/// Conversion
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Conversion {
    from: String,
    to: String,
    amount: Decimal,
}

///
/// Conversion response
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ConversionResponse {
    id: String,
    amount: String,
    from_account_id: String,
    to_account_id: String,
    from: String,
    to: String,
}

impl Conversion {
    pub fn new(from: &str, to: &str, amount: Decimal) -> Self {
        Conversion {
            from: from.to_string(),
            to: to.to_string(),
            amount,
        }
    }
}
