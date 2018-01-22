use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct UrlParameters {
    parameters: String,
}

impl UrlParameters {
    pub fn new<K: fmt::Display, V: fmt::Display>(key: &K, value: &V) -> Self {
        UrlParameters {
            parameters: format!("{}={}", &key, &value),
        }
    }

    pub fn append<K: fmt::Display, V: fmt::Display>(&mut self, key: &K, value: &V) {
        if !self.parameters.is_empty() {
            self.parameters.push_str("&");
        }
        self.parameters.push_str(&format!("{}={}", &key, &value));
    }
}

impl fmt::Display for UrlParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parameters)
    }
}

pub fn build_signed_request(mut url_params: UrlParameters, recv_window: Option<u64>) -> String {
    if let Some(window) = recv_window {
        url_params.append(&"recvWindow", &window.to_string());
    }
    url_params.append(&"timestamp", &get_timestamp().to_string());
    url_params.to_string()
}

fn get_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000
}
