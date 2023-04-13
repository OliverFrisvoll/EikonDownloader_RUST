use crate::timeseries::{Frequency, TimeSeries};
use crate::ts_get::event_ts;
use crate::connection::Connection;
use crate::datagrid::Datagrid;
use std::collections::HashMap;
use chrono::prelude::*;


mod connection;
mod datagrid;
mod timeseries;
mod ts_get;
mod utils;


fn main() -> () {
    let api = "f63dab2c283546a187cd6c59894749a2228ce486";
    let mut ek = Connection::new(api.to_string(), "127.0.0.1".to_string(), 9000);
    let r = ek.query_port().unwrap();
    println!("PORT {}", r);

    // println!("{}", r.unwrap());

    let ts = TimeSeries::new(ek);

    let SDate = NaiveDateTime::parse_from_str("2022-05-01T00:00:00", "%FT%T")
        .unwrap();


    let EDate = NaiveDateTime::parse_from_str("2023-01-08T00:00:00", "%FT%T")
        .unwrap();

    let df = ts.get_timeseries(
        vec!["GME".to_string(), "XOM".to_string()],
        vec!["*".to_string()],
        Frequency::new("minute"),
        SDate,
        EDate,
    );

    println!("{}", df.unwrap());


    let mut ek = Connection::new(api.to_string(), "127.0.0.1".to_string(), 9000);
    let dg = Datagrid::new(ek);

    let mut params: HashMap<String, String> = HashMap::new();
    params.insert(String::from("EDate"), String::from("2023-01-04"));
    params.insert(String::from("SDate"), String::from("1970-01-01"));
    params.insert(String::from("Frq"), String::from("D"));

    let df = dg
        .get_datagrid(
            vec![String::from("ARR"), String::from("AAPL.O"), String::from("XOM"), String::from("GME")],
            vec![String::from("TR.CLOSE"), String::from("TR.VOLUME"), String::from("TR.CLOSE.DATE")],
            Some(params))
        .unwrap();
    println!("{}", df.unwrap());
}