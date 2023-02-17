use polars::export::chrono;
use reqwest::{Client, Error, Response};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use futures::io::Cursor;
use polars::export::ahash::{HashMap, HashMapExt};
use polars::prelude::*;
use serde_json::de::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Field {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DgRequest {
    instruments: Vec<String>,
    fields: Vec<Field>,
    parameters: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataGrid {
    requests: Vec<DgRequest>,
}



#[derive(Serialize, Deserialize, Debug)]
struct TimeSeries {
    rics: Vec<String>,
    fields: Vec<String>,
    interval: String,
    startdate: DateTime<Utc>,
    enddate: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Load<T> {
    E: String,
    W: T,
}


#[derive(Serialize, Deserialize, Debug)]
struct JsonBody<T> {
    Entity: Load<T>,
}


async fn get_timeseries(app_key: &str) -> PolarsResult<DataFrame> {
    // let url = "http://127.0.0.1:9000/api/v1/data";
    // let location = "/v1/data";
    // let address = concat!(&url, &location);

    let directions = String::from("TimeSeries");

    let body = JsonBody::<TimeSeries> {
        Entity: Load {
            E: directions,
            W: TimeSeries {
                rics: vec![String::from("AAPL.O")],
                fields: vec![String::from("*")],
                interval: String::from("daily"),
                startdate: Utc::now() - chrono::Duration::days(10),
                enddate: Utc::now(),
            },
        },
    };

    let res = send_request_ts(body, app_key)
        .await
        .expect("Could not send request");

    timeseries_to_df(res)
}

async fn send_request_ts(body: JsonBody<TimeSeries>, app_key: &str) -> reqwest::Result<serde_json::Value> {
    let url = "http://127.0.0.1:9000/api/v1/data";

    let client = Client::new();
    return match client.post(url)
        .header("CONTENT_TYPE", "application/json")
        .header("X-tr-applicationid", app_key)
        .json(&body)
        .send()
        .await {
        Ok(R) => { R.json().await }
        Err(E) => { Err(E) }
    };
}

async fn send_request_dg(body: JsonBody<DataGrid>, app_key: &str) -> reqwest::Result<serde_json::Value> {
    let url = "http://127.0.0.1:9000/api/v1/data";

    let client = Client::new();
    return match client.post(url)
        .header("CONTENT_TYPE", "application/json")
        .header("X-tr-applicationid", app_key)
        .json(&body)
        .send()
        .await {
        Ok(R) => { R.json().await }
        Err(E) => { Err(E) }
    };
}


async fn get_datagrid(app_key: &str) -> () {
    let directions = String::from("DataGrid_StandardAsync");

    let mut parameters: HashMap<String, String> = HashMap::new();
    parameters.insert(String::from("SDate"), String::from("2020-01-01"));
    parameters.insert(String::from("EDate"), String::from("2023-01-01"));
    parameters.insert(String::from("Frq"), String::from("M"));

    let mut instruments: Vec<String> = Vec::new();
    instruments.push(String::from("AAPL.O"));

    let mut fields: Vec<Field> = Vec::new();
    fields.push(Field { name: String::from("TR.DividendYield") });
    fields.push(Field { name: String::from("TR.CLOSE") });


    let requests = vec![DgRequest {
        instruments,
        fields,
        parameters,
    }];

    let body = JsonBody::<DataGrid> {
        Entity: Load {
            E: directions,
            W: DataGrid {
                requests,
            },
        },
    };


    println!("{}", serde_json::to_string(&body).unwrap());

    let res = send_request_dg(body, app_key)
        .await
        .expect("Could not send request");

    println!("{:?}", res);
}


fn timeseries_to_df(json_like: serde_json::Value) -> PolarsResult<DataFrame> {
    let data = json_like["timeseriesData"][0]["dataPoints"].clone();
    let fields = json_like["timeseriesData"][0]["fields"].clone();

    let column_names: Vec<String> = fields
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x["name"].to_string())
        .collect();

    let mut future_df: Vec<Series> = Vec::new();

    for i in 0..column_names.len() {
        let mut ser: Vec<String> = Vec::new();
        println!("{:?}", column_names[i]);
        for row in data.as_array().unwrap() {
            ser.push(row[i].to_string());
        }
        future_df.push(Series::new(&*column_names[i], ser));
    }

    DataFrame::new(future_df)
}


async fn status(url: String) -> () {}

async fn handshake(url: String) -> () {}


#[tokio::main]
async fn main() -> () {
    let api = "f63dab2c283546a187cd6c59894749a2228ce486";

    let df = get_timeseries(api)
        .await
        .expect("Could not make df");

    println!("{}", df);

    get_datagrid(api).await;
}