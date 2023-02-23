use std::collections::HashMap;
use crate::connection::Connection;
use crate::datagrid::Datagrid;

// mod datagrid;
mod connection;
mod datagrid;


fn main() -> () {
    let api = "f63dab2c283546a187cd6c59894749a2228ce486";
    let ek = Connection::new(api);
    println!("{}", ek.status(&9000));

    let dg = Datagrid::new(ek);

    let mut params: HashMap<String, String> = HashMap::new();
    params.insert(String::from("EDate"), String::from("2023-01-04"));
    params.insert(String::from("SDate"), String::from("2023-01-01"));
    params.insert(String::from("Frq"), String::from("D"));

    let df = dg
        .get_datagrid(
            vec![String::from("ARR"), String::from("AAPL.O"), String::from("GOOGL.O"), String::from("XOM"), String::from("T"), String::from("GME"), String::from("TSLA.O")],
            vec![String::from("TR.CLOSE"), String::from("TR.VOLUME"), String::from("TR.CLOSE.DATE")],
            Some(params))
        .unwrap();
    println!("{}", df);
}