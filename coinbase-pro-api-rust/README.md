# coinbase-pro-api-rust: A simple wrapper for the Coinbase API
 
This crate, in tandem with the cbpro-automated-trading-framework crate, allows users to create simple trading strategies based on data from Coinbase Pro. In particular, this data consists of price updates, bids, asks, volume updates, and trade times—essentially, this is all the data that an order book consists of. Users can develop any function based on this data to indicate when they want trades to occur, and coinbase-pro-api-rust will do all the hard work of interacting with Coinbase Pro behind the scenes, collecting data, and sending requests.

# How it Works 

coinbase-pro-api-rust and cbpro-automated-trading-framework use both the Coinbase Pro HTTP REST API and Websocket API. The Websocket API is used to collect real-time data and the REST API is used to place orders and get account information if necessary. Neither of these are directly interacted with by a user, however. A user who would like to create a strategy simply needs to write a function based on the TradingData struct (containing order book data) and pass it into a Strategy struct. This Strategy struct will then listen for data on a Websocket and execute trades based on the strategy.

A strategy could be as simple as selling when a price goes above a certain number and buying when a price drops below a certain number:

~~~ 
fn strat (user: &AuthorizedClient, data: &TradingData) {
  if data.products["BTC-USD"].price > Decimal::new(30000, 0) {
    user.place_order(sell ... ); // sell BTC 
  } else if data.products["BTC-USD"].price < Decimal::new(20000, 0) {
    user.place_order(buy ... );
  }
}
~~~
Note that any indicators based on TradingData can be used to develop a strategy. 
Then, this "strategy function" is passed into a new Strategy struct, which has a name and authorized user associated with it:

~~~
let mut my_strategy = Strategy::new("Example Name", user, vec!["BTC-USD"], strat);
my_strategy.subscribe_to_ticker_data(); // start collecting data
my_strategy.run(); // initiate strategy based on function!
~~~

That's it! Easy, right? Happy coding! 

