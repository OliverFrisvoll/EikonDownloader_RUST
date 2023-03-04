use serde_json::Value;

pub fn event_ts() {
    let client = reqwest::blocking::Client::new();
    let ric = "ARR";
    let start = "2023-01-05T003%A00%3A00.000000000Z";
    let end = "2023-01-06T003%A00%3A00.000000000Z";

    // http://localhost:9000/api/rdp/data/historical-pricing/v1/views/events/ARR?start=2023-01-05T003%A00%3A00.000000000Z&end=2023-01-11T15%3A34%3A14.783000000Z
    // http://localhost:9001/api/rdp/data/historical-pricing/v1/views/events/ARR?start=2023-01-05T003%A00%3A00.000000000Z&end=2023-01-11T003%A00%3A00.000000000Z

    let url = format!("http://localhost:9001/api/rdp/data/historical-pricing/v1/views/events/{ric}?start={start}&end={end}", ric = ric, start = start, end = end);

    println!("{}", url);

    let res: Value = client.get(url)
        .header("X-tr-applicationid", "f63dab2c283546a187cd6c59894749a2228ce486")
        .send()
        .expect("")
        .json()
        .expect("");

    println!("{}", res)
}



