use std::collections::HashMap;
use std::cmp::min;
use serde_json::{json, Value};
use polars::prelude::*;
use chrono::prelude::*;
use crate::connection::Connection;
use crate::utils::{clean_string, EkResults, EkError};


pub struct Datagrid {
    connection: Connection,
}

impl Datagrid {
    pub fn new(c: Connection) -> Self {
        Self {
            connection: c
        }
    }

    fn assemble_payload(
        &self,
        instruments: Vec<String>,
        fields: &Vec<String>,
        param: &Option<HashMap<String, String>>,
    ) -> serde_json::Value {
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

        return res;
    }

    fn days_between(sdate: &String, edate: &String) -> chrono::Duration {
        let sdate = NaiveDate::parse_from_str(sdate, "%Y-%m-%d")
            .expect("Could not parse sdate (Datagrid::days_between)");
        let edate = NaiveDate::parse_from_str(edate, "%Y-%m-%d")
            .expect("Could not parse edate (Datagrid::days_between)");
        edate.signed_duration_since(sdate)
    }

    fn groups(parameters: &Option<HashMap<String, String>>) -> usize {
        let max_rows: usize = 50000;
        let max_instruments: usize = 7000;

        let max_group_size = match parameters {
            Some(param) => {
                if !param.contains_key("SDate") {
                    max_instruments
                } else {
                    let frq = match param.get("Frq") {
                        None => { String::from("d") }
                        Some(value) => { value.to_owned().to_lowercase() }
                    };
                    let start_date = match param.get("SDate") {
                        None => { panic!("You need to supply SDate as a parameter") }
                        Some(value) => {
                            NaiveDate::parse_from_str(value, "%Y-%m-%d")
                                .expect("Could not parse SDate string as Date, please supply a ISO8601 compliant Date format")
                        }
                    };
                    let end_date = match param.get("EDate") {
                        None => { Utc::now().date_naive() }
                        Some(value) => {
                            NaiveDate::parse_from_str(value, "%Y-%m-%d")
                                .expect("Could not parse EDate string as Date, please supply a ISO8601 compliant Date format")
                        }
                    };

                    let dur = end_date.signed_duration_since(start_date);

                    let rows_pr = match frq.as_str() {
                        "m" | "am" | "fs" | "fh" | "fq" | "aq" | "q" | "cm" | "ch" | "cs" | "cq" => { (dur.num_days() as usize) / 365 * 12 }
                        "y" | "fy" | "ay" | "f" | "cy" => { (dur.num_days() as usize) / 365 }
                        _ => { dur.num_days() as usize }
                    };

                    min((max_rows as f32 / rows_pr as f32).floor() as usize, max_instruments)
                }
            }
            None => { max_instruments }
        };

        max_group_size
    }

    pub fn get_datagrid(
        &self,
        instruments: Vec<String>,
        fields: Vec<String>,
        parameters: Option<HashMap<String, String>>,
        settings: HashMap<String, bool>,
    ) -> EkResults {
        let direction = "DataGrid_StandardAsync";
        let group_size = Datagrid::groups(&parameters);
        println!("Groups: {}", group_size);

        let mut payloads: Vec<Value> = Vec::new();

        for chunk in instruments.chunks(group_size).into_iter() {
            let inst_chunk = chunk.to_vec();
            payloads.push(self.assemble_payload(inst_chunk, &fields, &parameters));
        }

        let res = match self.connection.send_request_async_handler(payloads, direction) {
            Ok(r) => { r }
            Err(e) => { return EkResults::Err(e); }
        };

        if res.is_empty() {
            return EkResults::Err(EkError::NoData("No data returned from Refinitiv".to_string()));
        }

        if *settings.get("raw").unwrap_or(&false) {
            EkResults::Raw(res)
        } else {
            let field_name = settings.get("field_name").unwrap_or(&false);
            match Datagrid::to_dataframe(res, field_name) {
                Ok(r) => { EkResults::DF(r) }
                Err(e) => { EkResults::Err(e) }
            }
        }
    }


    fn fetch_headers(json_like: &Value, field_name: &bool) -> Option<Vec<String>> {
        let headers = match json_like["responses"][0]["headers"][0]
            .as_array() {
            None => { return None; }
            Some(r) => { r }
        };

        let mut names: Vec<String> = Vec::new();
        // TODO, headers contains two fields displayName and field, The last one is not available for instrument
        for value in headers {
            names.push(clean_string(value["displayName"].to_string()))
        }
        Some(names)
    }


    fn to_dataframe(json_like: Vec<Value>, field_name: &bool) -> Result<DataFrame, EkError> {

        // Extract headers
        let mut found = false;
        let mut headers: Vec<String> = Vec::new();
        for request in &json_like {
            match Datagrid::fetch_headers(request, field_name) {
                None => { continue; }
                Some(r) => {
                    headers = r;
                    found = true;
                    break;
                }
            }
        }

        if !found {
            return Err(EkError::NoHeaders("Could not build headers".to_string()));
        }

        // Extract data, combine with headers to make a dataframe
        let mut df_vec: Vec<Series> = Vec::new();

        for col in 0..headers.len() {
            let mut ser: Vec<String> = Vec::new();
            for request in &json_like {
                let rows = match request["responses"][0]["data"]
                    .as_array() {
                    None => { continue; }
                    Some(r) => { r }
                };
                for row in rows {
                    ser.push(clean_string(row[col].to_string()));
                }
            }
            df_vec.push(Series::new(headers[col].as_str(), ser));
        }
        match DataFrame::new(df_vec) {
            Ok(r) => { Ok(r) }
            Err(e) => { Err(EkError::NoDataFrame("Could not parse as Polars df".to_string())) }
        }
    }
}

