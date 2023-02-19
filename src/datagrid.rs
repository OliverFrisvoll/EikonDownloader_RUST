use std::collections::HashMap;
use std::fmt::Error;
use serde_json::json;

use chrono::prelude::*;
use futures::task::Spawn;
use reqwest::Response;
use polars::prelude::*;
use crate::connection::Connection;


pub struct Datagrid {
    connection: Connection,
}

impl Datagrid {
    pub fn new(c: Connection) -> Self {
        Self {
            connection: c
        }
    }

    fn assemble_payload(&self, instruments: Vec<String>,
                        fields: Vec<String>,
                        param: Option<HashMap<String, String>>, ) -> serde_json::Value {
        let fields_formatted: Vec<serde_json::Value> = fields
            .iter()
            .map(|x| json!({"name": x}))
            .collect();

        let res = match param {
            None => {
                json!(
                    {
                        "requests": [{
                            "instruments": instruments,
                            "fields": fields_formatted,
                        }]
                    }
                )
            }
            Some(p) => {
                json!(
                    {
                        "requests": [{
                            "instruments": instruments,
                            "fields": fields_formatted,
                            "parameters": p
                        }]
                    }
                )
            }
        };

        println!("Test, Payload: {}", res.to_string());

        return res;
    }

    pub async fn get_datagrid(&self, instruments: Vec<String>,
                              fields: Vec<String>,
                              parameters: Option<HashMap<String, String>>) -> PolarsResult<DataFrame> {
        let direction = String::from("DataGrid_StandardAsync");
        let max_rows = 50000;
        let max_ric = 7000;

        let payload = self.assemble_payload(instruments, fields, parameters);

        let res = self.connection.send_request(payload, direction)
            .await
            .unwrap();

        // println!("{:?}", res);

        self.to_data_frame(res)
    }

    fn to_data_frame(&self, json_like: serde_json::Value) -> PolarsResult<DataFrame> {
        println!("Test, Json_like{:?}", json_like.to_string());

        // Extract headers
        let headers: Vec<String> = json_like["responses"][0]["headers"][0]
            .as_array()
            .expect("Could not unwrap headers in json, (to_data_frame)")
            .iter()
            .map(|x| x["displayName"].to_string())
            .collect();

        // Extract data, combine with headers to make a dataframe
        let mut df_vec: Vec<Series> = Vec::new();

        for col in 0..headers.len() {
            let mut ser: Vec<String> = Vec::new();

            for row in json_like["responses"][0]["data"]
                .as_array()
                .expect("Could not unwrap row in json, (to_data_frame)") {
                ser.push(row[col].to_string());
            }
            df_vec.push(Series::new(&*headers[col], ser));
        }

        DataFrame::new(df_vec)
    }
}

