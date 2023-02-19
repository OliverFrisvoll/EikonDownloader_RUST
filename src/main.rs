use std::collections::HashMap;
use polars::export::chrono;
use chrono::prelude::*;
use crate::connection::Connection;
use crate::datagrid::Datagrid;

// mod datagrid;
mod connection;
mod datagrid;


#[tokio::main]
async fn main() -> () {
    let api = "f63dab2c283546a187cd6c59894749a2228ce486";
    let ek = Connection::new(api);
    println!("{}", ek.status(&9000).await);

    let dg = Datagrid::new(ek);

    let mut params: HashMap<String, String> = HashMap::new();
    params.insert(String::from("EDate"), String::from("2023-01-01"));
    params.insert(String::from("SDate"), String::from("2020-01-01"));

    let df = dg
        .get_datagrid(
            vec![String::from("ARR"), String::from("AAPL.O")],
            vec![String::from("TR.CLOSE"), String::from("TR.VOLUME")],
            Some(params))
        .await
        .unwrap();
    println!("{}", df);
}