use cbpro_automated_trading_framework::{Strategy, TradingData};
use coinbase_pro_api_rust::client::AuthorizedClient;
use coinbase_pro_api_rust::orders::Order;
use rust_decimal::prelude::{Decimal, FromStr};

fn main() {
    let my_user = AuthorizedClient::new(
                    "https://api-public.sandbox.pro.coinbase.com",
                    "93fb3fb63801d68af56a4c8aee61aec0",
                    "sandbox",
                    "P7hJ+jTTOJpx2nlviiFyPWxRT3F1rQcZ/onLTGjNxDCRE5DtqHde+SOMlOepwVJlSWbY76Bjg1E5h8btA6U/wg==");
    
    struct MyData {
        trades_made: u32,
    }

    let my_data = MyData { trades_made: 0 };

    fn my_strat(user: &AuthorizedClient, data: &mut TradingData<MyData>) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let action_block = async {
            if data.products["BTC-USD"].price > Decimal::new(30000, 0) && data.user_data.trades_made < 5 {
                user.place_order(Order::MarketOrder, "sell", "BTC-USD", None, "0.001").await;
                println!("Placed sell order #{}", data.user_data.trades_made);
                data.user_data.trades_made += 1;
            }
        };
        rt.block_on(action_block);
    }

    let mut my_strategy = Strategy::new(
                    "Ben's simple strategy",
                    my_user,
                    vec!["BTC-USD"],
                    my_strat,
                    my_data,
    );
    my_strategy.subscribe_to_ticker_data();
    my_strategy.run();
}

