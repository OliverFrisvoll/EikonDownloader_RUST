use std::fmt;
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
            _ => write!(f, "Unknown error (Should not happen)")
        }
    }
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

