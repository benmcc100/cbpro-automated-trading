use serde::{Deserialize, Serialize};
///
/// Account struct - describes client accounts
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    id: String,
    currency: String,
    balance: String,
    available: String,
    hold: String,
    profile_id: String,
    trading_enabled: bool,
}
