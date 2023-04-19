use crate::timeseries::{Interval, TimeSeries};
use crate::connection::Connection;
use crate::datagrid::Datagrid;
use crate::utils::{EkResults, field_builder, Fields};
use std::collections::HashMap;
use chrono::prelude::*;
use crate::utils::Fields::{NoParams, Params};


mod connection;
mod datagrid;
mod timeseries;
mod utils;


fn main() -> () {
    let api = "f63dab2c283546a187cd6c59894749a2228ce486";
    let mut ek = Connection::new(api.to_string(), "127.0.0.1".to_string(), 9000);

    let ts = TimeSeries::new(ek);

    let SDate = NaiveDateTime::parse_from_str("1920-01-01T00:00:00", "%FT%T")
        .unwrap();
    let EDate = NaiveDateTime::parse_from_str("2023-01-01T00:00:00", "%FT%T")
        .unwrap();

    match ts.get_timeseries(
        vec!["US10YT=RR".to_string(), "DE2YT=RR".to_string(), "XOM".to_string()],
        vec!["*".to_string()],
        Interval::new("daily"),
        SDate,
        EDate,
    ) {
        EkResults::DF(df) => println!("{}", df),
        EkResults::Raw(r) => println!("{:?}", r),
        EkResults::Err(e) => println!("{}", e.to_string())
    };


    let mut ek = Connection::new(api.to_string(), "127.0.0.1".to_string(), 9000);
    let dg = Datagrid::new(ek);

    let mut params: HashMap<String, String> = HashMap::new();
    params.insert(String::from("EDate"), String::from("2002-02-10"));
    params.insert(String::from("SDate"), String::from("2002-01-01"));
    params.insert(String::from("Frq"), String::from("D"));

    let mut settings: HashMap<String, bool> = HashMap::new();
    settings.insert("raw".to_string(), false);
    settings.insert("field_name".to_string(), false);


    let mut field = HashMap::new();
    let mut param = HashMap::new();
    param.insert("Curn".to_string(), "EUR".to_string());
    field.insert("TR.GrossProfit".to_string(), param);
    let mut param = HashMap::new();
    param.insert("Curn".to_string(), "EUR".to_string());
    field.insert("TR.CLOSE".to_string(), param);
    let field_json = field_builder(Params(field));

    let fields = field_builder(NoParams(vec![String::from("TR.CLOSE"), String::from("TR.VOLUME"), String::from("TR.CLOSE.DATE")]));
    let fields = field_builder(NoParams(vec![String::from("TR.CLOSE"), String::from("TR.VOLUME"), String::from("TR.CLOSE.DATE")]));
    match dg.get_datagrid(
        vec![String::from("XOM"), String::from("GME")],
        field_json,
        Some(params),
        settings,
    ) {
        EkResults::DF(df) => println!("{}", df),
        EkResults::Raw(r) => println!("{:?}", r),
        EkResults::Err(e) => println!("{}", e.to_string())
    };
}

