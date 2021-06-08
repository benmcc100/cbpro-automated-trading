use crate::client::AuthorizedClient;
use serde_json::Value;

pub struct Changes {
    pub side: String,
    pub price: String,
    pub size: String,
}

pub struct SocketQuery {
    client: AuthorizedClient,
    open: bool,
}

impl SocketQuery {
    pub fn new(client: AuthorizedClient) -> SocketQuery {
        SocketQuery {
            client,
            open: false,
        }
    }

    pub fn open_level2(&mut self, product_ids: Vec<&str>) {
        if self.client.is_connected() {
            self.client.subscribe(product_ids, vec!["level2"]);
            self.open = true
        } else {
            self.client.connect_socket();
            self.client.subscribe(product_ids, vec!["level2"]);
            self.open = true
        }
    }

    pub fn close(&mut self) {
        self.client.disconnect_socket();
        self.open = false
    }

    pub fn get_order_book(&mut self) -> Option<Changes> {
        if self.open {
            if let Some(val) = self.client.read_from_ws().unwrap().into_text().ok() {
                let json_val: Value = serde_json::from_str(&val).unwrap();
                if let Some(changes) = json_val["changes"][0].as_array() {
                    if changes.len() == 3 {
                        Some(Changes {
                            side: changes[0].to_string(),
                            price: changes[1].to_string(),
                            size: changes[2].to_string(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            println!("NOT CONNECTED TO SOCKET");
            None
        }
    }
}
