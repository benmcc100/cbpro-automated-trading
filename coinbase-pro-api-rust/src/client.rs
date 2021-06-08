use crate::accounts::Account;
use crate::conversion::{Conversion, ConversionResponse};
use crate::errors::RequestError;
use crate::orders::{LimitOrder, MarketOrder, OpenOrder, Order, OrderResponse};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use native_tls::TlsStream;
use reqwest::{Client, RequestBuilder};
use rust_decimal::prelude::{Decimal, FromStr};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use tungstenite::{connect, stream::Stream, Message, Result as TungsteniteResult, WebSocket};
use url::Url;

///
/// Empty request body
/// 
#[derive(Serialize)]
struct MtBody {}

impl MtBody {
    fn new() -> Self {
        MtBody {}
    }
}

///
/// Manages user credentials and requests
///
pub struct AuthorizedClient {
    url: String,
    access_key: String,
    passphrase: String,
    secret: String,
    client: Client,
    socket: Option<WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>>,
}

impl AuthorizedClient {
    pub fn new(url: &str, access_key: &str, passphrase: &str, secret: &str) -> AuthorizedClient {
        AuthorizedClient {
            url: url.to_string(),
            access_key: access_key.to_string(),
            passphrase: passphrase.to_string(),
            secret: secret.to_string(),
            client: reqwest::Client::new(),
            socket: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.socket.is_none()
    }

    ///
    /// Performs currency conversion
    ///
    pub async fn convert(
        &self,
        from: &str,
        to: &str,
        amount: &str,
    ) -> Result<ConversionResponse, RequestError> {
        let conversion = Conversion::new(
            from,
            to,
            Decimal::from_str(amount)
                .or_else(|_| Err(RequestError::InvalidRequest("invalid amount".to_string())))?,
        );
        self.make_request("POST", "/conversions", &conversion).await
    }
    ///
    /// Gets existing orders - by id
    ///
    pub async fn get_order(&self, id: &str) -> Result<OpenOrder, RequestError> {
        let method = format!("/orders/{}", id);
        self.make_request("GET", &method, MtBody::new()).await
    }

    ///
    /// Gets existing orders - limiting query to orders with given status
    ///
    pub async fn get_orders(&self, statuses: &[&str]) -> Result<Vec<OpenOrder>, RequestError> {
        let method_and_queries = self.form_method_and_queries("/orders", statuses);
        self.make_request("GET", &method_and_queries, MtBody::new())
            .await
    }

    fn form_method_and_queries(&self, method: &str, queries: &[&str]) -> String {
        let mut method_and_queries = method.to_string();
        let mut iter = queries.iter();
        if let Some(status) = iter.next() {
            method_and_queries.push_str("?status=");
            method_and_queries.push_str(status);
            while let Some(status) = iter.next() {
                method_and_queries.push_str("&status=");
                method_and_queries.push_str(status);
            }
        }
        method_and_queries
    }

    ///
    /// Places user order - supports both market and limit orders
    ///
    pub async fn place_order(
        &self,
        r#type: Order,
        side: &str,
        product_id: &str,
        price: Option<&str>,
        size: &str,
    ) -> Result<OrderResponse, RequestError> {
        match r#type {
            Order::MarketOrder => {
                let order = MarketOrder::new(
                    "market".to_string(),
                    Decimal::from_str(size).or_else(|_| {
                        Err(RequestError::InvalidRequest("invalid size".to_string()))
                    })?,
                    side.to_string(),
                    product_id.to_string(),
                );
                self.make_request("POST", "/orders", &order).await
            }
            Order::LimitOrder => {
                let order = LimitOrder::new(
                    "limit".to_string(),
                    Decimal::from_str(price.ok_or(RequestError::InvalidRequest(
                        "invalid price for limit order".to_string(),
                    ))?)
                    .or_else(|e| Err(RequestError::InvalidRequest("invalid price".to_string())))?,
                    Decimal::from_str(size).or_else(|_| {
                        Err(RequestError::InvalidRequest("invalid size".to_string()))
                    })?,
                    side.to_string(),
                    product_id.to_string(),
                );
                self.make_request("POST", "/orders", &order).await
            }
        }
    }

    ///
    /// Retrieves user account information
    ///
    pub async fn get_accounts(&self) -> Result<Vec<Account>, RequestError> {
        self.make_request("GET", "/accounts", MtBody::new()).await
    }

    ///
    /// Parses HTTP request error messages
    ///
    fn parse_request_error(&self, text: &str) -> Option<String> {
        let message: Result<HashMap<String, String>, serde_json::Error> =
            serde_json::from_str(text);
        match message {
            Ok(m) => Some(m.get("message").unwrap().to_string()),
            Err(_) => None,
        }
    }
    ///
    /// Makes HTTP request - serializes/deserializes data structures
    ///
    async fn make_request<T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: impl Serialize,
    ) -> Result<T, RequestError> {
        let mut body_text = serde_json::to_string(&body).or_else(|_| {
            Err(RequestError::InvalidRequest(
                "request couldn't be serialized".to_string(),
            ))
        })?;

        if body_text == "{}" {
            body_text = "".to_string();
        }
        let response_text = self
            .form_request(method, path, &body_text)
            .ok_or(RequestError::InternalError(
                "couldn't form request".to_string(),
            ))?
            .send()
            .await
            .or_else(|_| Err(RequestError::NetworkError))?
            .text()
            .await
            .or_else(|_| {
                Err(RequestError::InternalError(
                    "couldn't convert response to raw text".to_string(),
                ))
            })?;
        let response: T = serde_json::from_str(&response_text).or_else(|_| {
            Err(RequestError::InvalidRequest(
                self.parse_request_error(&response_text)
                    .ok_or(RequestError::InternalError(
                        "couldn't deserialize response".to_string(),
                    ))?,
            ))
        })?;
        Ok(response)
    }

    ///
    /// Forms HTTP request - using user-provided API key/passcode/etc
    ///
    fn form_request(&self, method: &str, path: &str, body: &str) -> Option<RequestBuilder> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let signature = self.encode_signature(&time, method, path, body);
        let headers = self.make_request_headers(time, &signature);
        match method {
            "GET" => Some(
                self.client
                    .get(self.url.to_string() + path)
                    .headers(headers),
            ),
            "POST" => Some(
                self.client
                    .post(self.url.to_string() + path)
                    .headers(headers)
                    .body(body.to_string()),
            ),
            _ => None,
        }
    }

    //// websocket api

    /// This subscribes to channel(s) just like subscribe(), but does so as an authenticated user,
    /// which gives data about your own orders and allows for more requests/second.
    /// This isn't working for some reason, but it should. The socket says "API key invalid" even
    /// thought it isn't...
    pub fn authenticated_subscribe(&mut self, product_ids: Vec<&str>, channels: Vec<&str>) {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let sig = self.encode_signature(&time, "GET", "/accounts", "");
        if let Some(s) = self.socket.as_mut() {
            let request = json!({
                "type": "subscribe",
                "product_ids": product_ids,
                "channels": channels,
                "signature": sig,
                "key": self.access_key,
                "passphrase": self.passphrase,
                "timestamp": time,
            });

            println!("{}", request);

            s.write_message(Message::Text(request.to_string())).unwrap();
        } else {
            println!("NOT CONNECTED TO SOCKET")
        }
    }

    pub fn connect_socket(&mut self) {
        if self.socket.is_none() {
            let (s, _response) = connect(Url::parse("wss://ws-feed.pro.coinbase.com").unwrap())
                .expect("Can't connect to Websocket");
            self.socket = Some(s);
        } else {
            println!("ALREADY CONNECTED TO SOCKET");
        }
    }

    pub fn disconnect_socket(&mut self) {
        if let Some(s) = self.socket.as_mut() {
            s.close(None);
        } else {
            println!("ALREADY DISCONNECTED FROM SOCKET");
        }
    }

    fn _send_subscription(&mut self, sub_type: &str, product_ids: Vec<&str>, channels: Vec<&str>) {
        if let Some(s) = self.socket.as_mut() {
            let request = json!({
                "type": sub_type,
                "product_ids": product_ids,
                "channels": channels
            });

            s.write_message(Message::Text(request.to_string())).unwrap();
        } else {
            println!("NOT CONNECTED TO SOCKET")
        }
    }

    pub fn subscribe(&mut self, product_ids: Vec<&str>, channels: Vec<&str>) {
        self._send_subscription("subscribe", product_ids, channels);
    }

    pub fn unsubscribe(&mut self, product_ids: Vec<&str>, channels: Vec<&str>) {
        self._send_subscription("unsubscribe", product_ids, channels);
    }

    pub fn read_from_ws(&mut self) -> TungsteniteResult<Message> {
        if let Some(s) = self.socket.as_mut() {
            let msg = s.read_message().expect("Error reading message");
            Ok(msg)
        } else {
            Err(tungstenite::Error::Io(std::io::Error::new(
                ErrorKind::NotConnected,
                "NOT CONNECTED TO SOCKET",
            )))
        }
    }

    /// Read text from websocket
    pub fn print_from_ws(&mut self) -> Option<String> {
        self.read_from_ws().unwrap().into_text().ok()
    }

    /// Generate HMAC signature for authenticating our API calls
    fn encode_signature(&self, time: &u64, method: &str, path: &str, body: &str) -> String {
        let decoded_secret = base64::decode(&*self.secret).unwrap();
        let mut hmac = Hmac::new(Sha256::new(), &decoded_secret);
        let what = &*(time.to_string() + method + path + body);
        hmac.input(what.as_bytes());

        base64::encode(hmac.result().code())
    }

    /// Generate correct headers based on HMAC signature for GET/POST requests
    fn make_request_headers(&self, time: u64, signature: &String) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "CB-ACCESS-SIGN",
            reqwest::header::HeaderValue::from_str(signature).unwrap(),
        );
        headers.insert(
            "CB-ACCESS-TIMESTAMP",
            reqwest::header::HeaderValue::from(time),
        );
        headers.insert(
            "CB-ACCESS-KEY",
            reqwest::header::HeaderValue::from_str(&self.access_key).unwrap(),
        );
        headers.insert(
            "CB-ACCESS-PASSPHRASE",
            reqwest::header::HeaderValue::from_str(&self.passphrase).unwrap(),
        );
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        headers
    }
}
