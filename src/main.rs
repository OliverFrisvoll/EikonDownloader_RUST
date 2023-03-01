use std::collections::HashMap;
use crate::connection::Connection;
use crate::datagrid::Datagrid;
use crate::timeseries::{TimeSeries, Frequency};
use chrono::prelude::*;


mod connection;
mod datagrid;
mod timeseries;


fn main() -> () {
    let api = "f63dab2c283546a187cd6c59894749a2228ce486";
    let mut ek = Connection::new(api.to_string(), "127.0.0.1".to_string(), 9001);
    let r = ek.query_port();

    println!("{}", r.unwrap());
    // println!("{}", ek.status(&9001));
    //
    //
    // let ts = TimeSeries::new(ek);
    //
    // let SDate = NaiveDateTime::parse_from_str("2015-01-01T00:00:00", "%FT%T")
    //     .unwrap();
    //
    //
    // let EDate = NaiveDateTime::parse_from_str("2023-01-04T00:00:00", "%FT%T")
    //     .unwrap();
    //
    // let df = ts.get_timeseries(
    //     vec!["ARR".to_string(), "GME".to_string(), "XOM".to_string()],
    //     vec!["*".to_string()],
    //     Frequency::new("minute"),
    //     SDate,
    //     EDate,
    // );
    //
    // println!("{}", df.unwrap())
    //
    // let dg = Datagrid::new(ek);
    //
    // let mut params: HashMap<String, String> = HashMap::new();
    // params.insert(String::from("EDate"), String::from("2023-01-04"));
    // params.insert(String::from("SDate"), String::from("2023-01-01"));
    // params.insert(String::from("Frq"), String::from("D"));
    //
    // let df = dg
    //     .get_datagrid(
    //         vec![String::from("ARR"), String::from("AAPL.O"), String::from("GOOGL.O"), String::from("XOM"), String::from("T"), String::from("GME"), String::from("TSLA.O")],
    //         vec![String::from("TR.CLOSE"), String::from("TR.VOLUME"), String::from("TR.CLOSE.DATE")],
    //         Some(params))
    //     .unwrap();
    // println!("{}", df);
}