use crate::timeseries::{Frequency, TimeSeries};
use crate::connection::Connection;
use crate::datagrid::Datagrid;
use crate::utils::{EkError, EkResults};
use std::collections::HashMap;
use chrono::prelude::*;


mod connection;
mod datagrid;
mod timeseries;
// mod ts_get;
mod utils;


fn main() -> () {
    let api = "f63dab2c283546a187cd6c59894749a2228ce486";
    let mut ek = Connection::new(api.to_string(), "127.0.0.1".to_string(), 9000);
    // let r = ek.query_port().unwrap();
    // println!("PORT {}", r);

    // println!("{}", r.unwrap());

    let ts = TimeSeries::new(ek);

    let SDate = NaiveDateTime::parse_from_str("1970-01-01T00:00:00", "%FT%T")
        .unwrap();


    let EDate = NaiveDateTime::parse_from_str("1975-01-01T00:00:00", "%FT%T")
        .unwrap();

    match ts.get_timeseries(
        vec!["US10YT=RR".to_string(), "DE2YT=RR".to_string()],
        vec!["*".to_string()],
        Frequency::new("daily"),
        SDate,
        EDate,
    ) {
        EkResults::DF(v) => { println!("{}", v) }
        EkResults::Raw(r) => { println!("{:?}", r) }
        EkResults::Err(_) => {}
    };
    //
    // println!("{}", df.unwrap());


    let mut ek = Connection::new(api.to_string(), "127.0.0.1".to_string(), 9000);
    let dg = Datagrid::new(ek);

    let mut params: HashMap<String, String> = HashMap::new();
    params.insert(String::from("EDate"), String::from("2023-01-04"));
    params.insert(String::from("SDate"), String::from("1970-01-01"));
    params.insert(String::from("Frq"), String::from("D"));

    let mut settings: HashMap<String, bool> = HashMap::new();
    settings.insert("raw".to_string(), true);

    match dg.get_datagrid(
        vec![String::from("ARR"), String::from("AAPL.O"), String::from("XOM"), String::from("GME")],
        vec![String::from("TR.CLOSE"), String::from("TR.VOLUME"), String::from("TR.CLOSE.DATE")],
        Some(params),
        settings,
    ) {
        EkResults::DF(df) => { println!("{}", df) }
        EkResults::Raw(r) => { println!("{:?}", r) }
        EkResults::Err(r) => {
            let msg = match r {
                EkError::NoData(v) => { v }
                EkError::NoHeaders(v) => { v }
                EkError::NoDataFrame(v) => { v }
                EkError::ConnectionError(v) => { v }
                EkError::ThreadError(v) => { v }
            };
            println!("{}", msg);
        }
    };

    // println!("{}", df);
}