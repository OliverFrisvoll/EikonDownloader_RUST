use polars::prelude::DataFrame;
use serde_json::Value;

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
    ConnectionError(String),
    ThreadError(String),
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
}

