# automated-trading-framework-full-code

How to try out:

1. Install Rust if necessary: https://www.rust-lang.org/tools/install
2. Clone repo
3. Navigate to cbpro-automated-trading-framework/examples/simple-bot
4. Open a terminal window in the simple-bot folder and enter 'cargo build'
5. When you're ready to execute the strategy, enter 'cargo run'

The code will be executed using the exchange account currently coded into the bot -- mine (this is a sandbox account, of course, so feel free to query it). To see it at work on your own account, go into the code at src/main.rs and edit the AuthorizedClient object to hold your own api key, passphrase, and secret code.
