use reqwest;
use serde_json;
use std;
use url;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        ParseIntervalError
    }

    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        UrlParserError(url::ParseError);
        SerdeJsonError(serde_json::Error);
    }
}
