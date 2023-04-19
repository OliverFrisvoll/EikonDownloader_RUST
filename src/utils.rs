use std::collections::HashMap;
use std::fmt;
use polars::prelude::*;
use serde_json::{Value, json};

pub fn clean_string(s: String) -> String {
    s.replace("\"", "")
}

pub enum EkResults {
    DF(DataFrame),
    Raw(Vec<Value>),
    Err(EkError),
}

pub enum EkError {
    NoData(String),
    NoHeaders(String),
    NoDataFrame(String),
    AuthError(String),
    ConnectionError(String),
    ThreadError(String),
    DateError(String),
    Error(String),
}

impl fmt::Display for EkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EkError::NoData(e) => write!(f, "No data returned: {}", e),
            EkError::NoHeaders(e) => write!(f, "No headers returned: {}", e),
            EkError::NoDataFrame(e) => write!(f, "No dataframe returned: {}", e),
            EkError::AuthError(e) => write!(f, "Authentication error: {}", e),
            EkError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            EkError::ThreadError(e) => write!(f, "Thread error: {}", e),
            EkError::DateError(e) => write!(f, "Date error: {}", e),
            EkError::Error(e) => write!(f, "Error: {}", e)
        }
    }
}

pub enum Fields {
    Params(HashMap<String, HashMap<String, String>>),
    NoParams(Vec<String>),
}

pub fn field_builder(fields: Fields) -> Value {
    let res = match fields {
        Fields::NoParams(fields) => {
            let mut res = Vec::with_capacity(fields.len());
            for f in fields.iter() {
                res.push(json!({"name": f}));
            }
            res
        }
        Fields::Params(fields) => {
            let mut res = Vec::with_capacity(fields.len());
            for (k, v) in fields.iter() {
                res.push(json!({"name": k, "parameters": v}));
            }
            res
        }
    };
    json!(res)
}

fn missing_in_vec<'a>(v1: Vec<&'a str>, v2: &Vec<&'a str>) -> Vec<&'a str> {
    let mut missing = Vec::new();
    for i in v1.into_iter() {
        if !v2.contains(&i) {
            missing.push(i);
        }
    }
    missing
}

fn create_series_String(header: &str, l: usize) -> Series {
    let value: Option<String> = None;
    let series = Series::new(header, vec![value; l]);
    series
}

pub fn vstack_diag(mut long: DataFrame, mut short: DataFrame) -> DataFrame {
    let long_col = long.get_column_names();
    let short_col = short.get_column_names();
    let missing = missing_in_vec(long_col, &short_col)
        .into_iter()
        .map(|x| create_series_String(x, short.shape().0))
        .collect::<Vec<Series>>();

    let short = short.with_column(missing[0].to_owned()).unwrap().to_owned();
    let long_col = long.get_column_names();
    let short = short.select(long_col).unwrap();
    long.vstack(&short).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_string() {
        let s = String::from("\"hello\"");
        let res = clean_string(s);
        assert_eq!(res, "hello");
    }

    #[test]
    fn test_field_builder() {
        let mut field = HashMap::new();
        let mut param = HashMap::new();
        param.insert("Scale".to_string(), "6".to_string());
        param.insert("Curn".to_string(), "EUR".to_string());
        field.insert("TR.GrossProfit".to_string(), param);
        let answer: Value = json!([{"name": "TR.GrossProfit", "parameters": {"Scale": "6", "Curn": "EUR"}}]);
        let res = field_builder(Fields::Params(field));
        assert_eq!(res, answer);

        let mut field = HashMap::new();
        let mut param = HashMap::new();
        param.insert("Scale".to_string(), "6".to_string());
        param.insert("Curn".to_string(), "EUR".to_string());
        field.insert("TR.GrossProfit".to_string(), param);
        let mut param = HashMap::new();
        param.insert("Curn".to_string(), "EUR".to_string());
        field.insert("TR.CLOSE".to_string(), param);
        let answer: Value = json!([{"name": "TR.GrossProfit", "parameters": {"Scale": "6", "Curn": "EUR"}}, {"name": "TR.CLOSE", "parameters": {"Curn": "EUR"}}]);
        let res = field_builder(Fields::Params(field));
        assert_eq!(res, answer);

        let fields = vec!["TR.GrossProfit".to_string(), "TR.CLOSE".to_string()];
        let answer: Value = json!([{"name": "TR.GrossProfit"}, {"name": "TR.CLOSE"}]);
        let res = field_builder(Fields::NoParams(fields));
        assert_eq!(res, answer);
    }
}

