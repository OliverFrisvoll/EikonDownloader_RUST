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

    let res = send_request_ts(body, app_key, 9000)
        .await
        .expect("Could not send request");

    timeseries_to_df(res)
}


async fn send_request_ts(body: JsonBody<TimeSeries>, app_key: &str, port: i16) -> reqwest::Result<serde_json::Value> {
    let ip = "http://127.0.0.1";
    let url = format!("{}:{}/api/v1/data", ip, port);

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

async fn send_request_dg(body: JsonBody<DataGrid>, app_key: &str, port: i16) -> Result<Response, Error> {
    let ip = "http://127.0.0.1";
    let url = format!("{}:{}/api/v1/data", ip, port);

    let client = Client::new();
    return client.post(url)
        .header("CONTENT_TYPE", "application/json")
        .header("X-tr-applicationid", app_key)
        .json(&body)
        .send()
        .await;
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
        for row in data.as_array().unwrap() {
            ser.push(row[i].to_string());
        }
        future_df.push(Series::new(&*column_names[i], ser));
    }

    DataFrame::new(future_df)
}


async fn handshake(url: String) -> () {}
