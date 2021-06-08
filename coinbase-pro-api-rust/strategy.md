Market Sell BTC $100 if:
* BTC price > x
* BTC volume < y

Market Buy BTC $500 if:
* MACD > 200MA

translates into

user = User::new()

loop {
        if user.get_price(btc) > x || user.get_volume(btc) {
            user.sell(usd_to_btc(100));
        }
        if user.get_macd(btc) > user.get_200ma(btc) {
            user.buy(usd_to_btc(500));
        }
    }