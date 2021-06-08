extern crate base64;
extern crate crypto;
extern crate data_encoding;
use coinbase_pro_api_rust::client::AuthorizedClient;
use coinbase_pro_api_rust::level2_feed::SocketQuery;
use coinbase_pro_api_rust::orders::Order;

#[tokio::main]
async fn main() {
    // let user = AuthorizedClient::new(
    //     // Insert credentials
    // );

    // println!("{:?}", user.get_accounts().await);
    // println!(
    //     "{:?}",
    //     user.place_order(Order::MarketOrder, "buy", "BTC-USD", None, "0.001")
    //         .await
    // );
    // // user.connect_socket();
    // // user.subscribe(vec!["BTC-USD"], vec!["heartbeat"]);
    // let mut sq = SocketQuery::new(user);
    // sq.open_level2(vec!["BTC-USD"]);

    // loop {
    //     if let Some(msg) = sq.get_order_book() {
    //         println!("{}", msg.price);
    //     }
    // }
}
