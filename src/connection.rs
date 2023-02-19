use std::fmt::format;
use std::future::Future;
use serde_json::json;

pub struct Connection {
    url: String,
    port: usize,
    app_key: String,
}

impl Connection {
    pub fn new(app_key: &str) -> Self {
        Self {
            url: "127.0.0.1".to_owned(),
            port: 9000,
            app_key: app_key.to_owned(),
        }
    }

    fn get_url(&self) -> String {
        format!("http://{}", self.url)
    }

    fn get_address(&self) -> String { format!("http://{}:{}", self.url, self.port) }

    fn get_app_key(&self) -> &String {
        &self.app_key
    }

    fn set_port(&mut self, port: usize) { self.port = port }

    pub async fn status(&self, port: &usize) -> u16 {
        let address = format!("{}:{}/api/status", self.get_url(), port);

        let client = reqwest::Client::new();
        client.get(address)
            .header("X-tr-applicationid", self.get_app_key())
            .send()
            .await
            .expect("Could not send request")
            .status()
            .as_u16()
    }

    pub async fn handshake(&self) -> serde_json::Value {
        // http://127.0.0.1:9000/api/handshake
        // http://127.0.0.1:9000/api/handshake
        // headers = {'Content-Type': 'application/json', 'x-tr-applicationid': 'f63dab2c283546a187cd6c59894749a2228ce486'}
        let address = format!("{}/api/handshake", self.get_address());

        let app_key = self.get_app_key();
        println!("{}", address);


        let json_body = json!({
            "AppKey": app_key,
            "AppScope": "trapi",
            "ApiVersion": "1"
        });

        let client = reqwest::Client::new();
        client.post(address)
            .header("CONTENT-TYPE", "application/json")
            .header("x-tr-applicationid", app_key)
            .body(json_body.to_string())
            .send()
            .await
            .expect("Could not handshake")
            .json()
            .await
            .expect("Could not parse as JSON")
    }

    pub async fn send_request(&self, payload: serde_json::Value, direction: String) -> reqwest::Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct FullRequest {
            Entity: Entity,
        }

        #[derive(serde::Serialize)]
        struct Entity {
            E: String,
            W: serde_json::Value,
        }

        let json_body = FullRequest {
            Entity: Entity {
                E: direction,
                W: payload,
            }
        };

        let app_key = self.get_app_key();

        let client = reqwest::Client::new();
        return match client
            .post(format!("{}/api/v1/data", self.get_address()))
            .header("CONTENT_TYPE", "application/json")
            .header("x-tr-applicationid", app_key)
            .json(&json_body)
            .send()
            .await {
            Ok(r) => { r.json().await }
            Err(e) => { Err(e) }
        };
    }

    async fn query_port(&mut self) {
        for port in 9000..9010usize {
            if self.status(&port).await == 200 {
                self.set_port(port);
                break;
            } else {
                continue;
            }
        }
    }
}
