use serde_json::{json, Value};
use reqwest::blocking::Response;
use std::{thread, time};
use crate::utils::{EkError};
use crate::utils::EkError::ConnectionError;

pub struct Connection {
    app_key: String,
    url: String,
    port: i16,
}

impl Connection {
    pub fn new(app_key: String, ip: String, port: i16) -> Self {
        Self {
            app_key: app_key.to_owned(),
            url: ip.to_owned(),
            port,
        }
    }

    fn get_url(&self) -> String {
        format!("http://{}", self.url)
    }

    fn get_address(&self) -> String { format!("http://{}:{}", self.url, self.port) }

    fn get_app_key(&self) -> &String {
        &self.app_key
    }

    pub fn set_port(&mut self, port: i16) { self.port = port }

    pub fn status(&self, port: &i16) -> reqwest::Result<Response> {
        let address = format!("{}:{}/api/status", self.get_url(), port);
        let client = reqwest::blocking::Client::new();
        client.get(address)
            .header("X-tr-applicationid", self.get_app_key())
            .send()
    }

    pub fn handshake(&self) -> serde_json::Value {
        let address = format!("{}/api/handshake", self.get_address());

        let app_key = self.get_app_key();
        println!("{}", address);


        let json_body = json!({
            "AppKey": app_key,
            "AppScope": "trapi",
            "ApiVersion": "1"
        });

        let client = reqwest::blocking::Client::new();
        client.post(address)
            .header("CONTENT-TYPE", "application/json")
            .header("x-tr-applicationid", app_key)
            .body(json_body.to_string())
            .send()
            .expect("Could not handshake")
            .json()
            .expect("Could not parse as JSON")
    }


    pub fn send_request(
        &self,
        payload: Value,
        direction: &String,
    ) -> reqwest::Result<Value> {
        #[derive(serde::Serialize)]
        struct FullRequest {
            Entity: Entity,
        }

        #[derive(serde::Serialize)]
        struct Entity {
            E: String,
            W: Value,
        }

        let json_body = FullRequest {
            Entity: Entity {
                E: direction.to_owned(),
                W: payload,
            }
        };

        let app_key = self.get_app_key();

        let client = reqwest::blocking::Client::new();
        return match client
            .post(format!("{}/api/v1/data", self.get_address()))
            .header("CONTENT_TYPE", "application/json")
            .header("x-tr-applicationid", app_key)
            .json(&json_body)
            .send() {
            Ok(r) => { r.json() }
            Err(e) => { Err(e) }
        };
    }

    pub fn send_request_async_handler(&self, payloads: Vec<Value>, direction: &str) -> Result<Vec<Value>, EkError> {
        let rt = match tokio::runtime::Builder::new_multi_thread()
            .worker_threads(12)
            .enable_all()
            .build() {
            Ok(r) => { r }
            Err(e) => { return Err(EkError::ThreadError(e.to_string())); }
        };

        let app_key = self.get_app_key();
        let address = self.get_address();
        let delay = time::Duration::from_millis(250);
        let payload_len = payloads.len();

        let mut handles = Vec::with_capacity(payloads.len());
        let mut res = Vec::new();

        for payload in payloads {
            thread::sleep(delay);
            handles.push(rt.spawn(Connection::send_request_async(
                payload,
                direction.to_owned(),
                address.to_owned(),
                app_key.to_owned(),
            )))
        }


        for handle in handles {
            match rt.block_on(handle) {
                Ok(r) => {
                    match r {
                        Ok(opt) => {
                            match opt {
                                Some(v) => { res.push(v) }
                                None => {}
                            }
                        }
                        Err(e) => { return Err(e); }
                    }
                }
                Err(e) => { return Err(EkError::ThreadError(e.to_string())); }
            }
        }
        return Ok(res);
    }

    pub async fn send_request_async(
        payload: Value,
        direction: String,
        address: String,
        app_key: String,
    ) -> Result<Option<Value>, EkError> {
        #[derive(serde::Serialize)]
        struct FullRequest {
            Entity: Entity,
        }

        #[derive(serde::Serialize)]
        struct Entity {
            E: String,
            W: Value,
        }

        let json_body = FullRequest {
            Entity: Entity {
                E: direction.to_owned(),
                W: payload.to_owned(),
            }
        };


        let client = reqwest::Client::new();
        match client
            .post(format!("{}/api/v1/data", address))
            .header("CONTENT_TYPE", "application/json")
            .header("x-tr-applicationid", app_key)
            .json(&json_body)
            .send()
            .await {
            Ok(v) => {
                match v.json::<Value>().await {
                    Ok(r) => {

                        // Me trying to catch if the request was successful
                        match r.get("timeseriesData") {
                            // If timeseries
                            Some(_) => {
                                Ok(Some(r))
                            }
                            // If datagrid
                            None => {
                                match r.get("responses") {
                                    None => { Ok(None) }
                                    Some(v) => {
                                        match v[0].get("data") {
                                            None => { Ok(None) }
                                            Some(_) => { Ok(Some(r)) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        Err(EkError::NoData(e.to_string()))
                    }
                }
            }
            Err(e) => {
                Err(ConnectionError(e.to_string()))
            }
        }
    }

    pub fn query_port(&self) -> Result<i16, ()> {
        for port in 9000..9010i16 {
            println!("Trying {}", port);

            match self.status(&port) {
                Ok(p) => { return Ok(port); }
                Err(e) => { continue; }
            }
        }
        return Err(());
    }
}
