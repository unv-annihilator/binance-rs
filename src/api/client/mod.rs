use errors::*;
use reqwest;
use reqwest::{Response, StatusCode};
use reqwest::header::*;
use std::io::Read;

static BASE_URL: &'static str = "https://api.binance.com";

#[derive(Clone, Debug, Default)]
pub struct Client {
    pub api_key: String,
    pub secret_key: String,
}

pub trait PublicClient {
    // Default Impl
    fn get(&self, endpoint: &str, parameters: &str) -> Result<(String)> {
        let mut url: String = format!("{}{}", BASE_URL, endpoint);
        if !parameters.is_empty() {
            url.push_str(&format!("?{}", parameters));
        }
        let response = reqwest::get(&url)?;

        self.handler(response)
    }

    fn handler(&self, mut response: Response) -> Result<(String)> {
        match response.status() {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                Ok(body)
            }
            StatusCode::InternalServerError => {
                bail!("Internal Server Error");
            }
            StatusCode::ServiceUnavailable => {
                bail!("Service Unavailable");
            }
            StatusCode::Unauthorized => {
                bail!("Unauthorized");
            }
            StatusCode::BadRequest => {
                bail!(format!("Bad Request: {:?}", response));
            }
            // Catch everything else
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        }
    }
}

pub trait ApiClient: PublicClient {
    fn get_with_key(&self, endpoint: &str, parameters: &str) -> Result<(String)>;
    fn post(&self, endpoint: &str) -> Result<(String)>;
    fn build_headers(&self, content_type: bool) -> Headers;
}

pub trait SignedClient: ApiClient {
    fn sign_request(&self, endpoint: &str, request: &str) -> String;
    fn get_signed(&self, endpoint: &str, request: &str) -> Result<(String)>;
    fn post_signed(&self, endpoint: &str, request: &str) -> Result<(String)>;
    fn delete_signed(&self, endpoint: &str, request: &str) -> Result<(String)>;
}

impl PublicClient for Client {}

impl ApiClient for Client {
    fn get_with_key(&self, endpoint: &str, parameters: &str) -> Result<(String)> {
        let mut url: String = format!("{}{}", BASE_URL, endpoint);
        if !parameters.is_empty() {
            url.push_str(&format!("?{}", parameters));
        }
        let client = reqwest::Client::new();
        let response = client.get(&url).headers(self.build_headers(false)).send()?;

        self.handler(response)
    }

    fn post(&self, endpoint: &str) -> Result<(String)> {
        //     pub fn post(&self, endpoint: &str) -> Result<(String)> {
        let url: String = format!("{}{}", BASE_URL, endpoint);

        let client = reqwest::Client::new();
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(false))
            .send()?;

        self.handler(response)
    }

    fn build_headers(&self, content_type: bool) -> Headers {
        let mut custom_headers = Headers::new();
        custom_headers.set(UserAgent::new("binance-rs"));
        if content_type {
            custom_headers.set(ContentType::form_url_encoded());
        }
        custom_headers.set_raw("X-MBX-APIKEY", self.api_key.as_str());

        custom_headers
    }
}

// impl Client {
// pub fn get_signed(&self, endpoint: &str, request: &str) ->
// Result<(String)> {         let url = self.sign_request(endpoint, request);
//         let client = reqwest::Client::new();
//         let response = client
//             .get(url.as_str())
//             .headers(self.build_headers(true))
//             .send()?;

//         self.handler(response)
//     }

// pub fn post_signed(&self, endpoint: &str, request: &str) ->
// Result<(String)> {         let url = self.sign_request(endpoint, request);
//         let client = reqwest::Client::new();
//         let response = client
//             .post(url.as_str())
//             .headers(self.build_headers(true))
//             .send()?;

//         self.handler(response)
//     }

// pub fn delete_signed(&self, endpoint: &str, request: &str) ->
// Result<(String)> {         let url = self.sign_request(endpoint, request);
//         let client = reqwest::Client::new();
//         let response = client
//             .delete(url.as_str())
//             .headers(self.build_headers(true))
//             .send()?;

//         self.handler(response)
//     }

//     pub fn post(&self, endpoint: &str) -> Result<(String)> {
//         let url: String = format!("{}{}", BASE_URL, endpoint);

//         let client = reqwest::Client::new();
//         let response = client
//             .post(url.as_str())
//             .headers(self.build_headers(false))
//             .send()?;

//         self.handler(response)
//     }

//     pub fn put(&self, endpoint: &str, listen_key: &str) -> Result<(String)> {
//         let url: String = format!("{}{}", BASE_URL, endpoint);
//         let data: String = format!("listenKey={}", listen_key);

//         let client = reqwest::Client::new();
//         let response = client
//             .put(url.as_str())
//             .headers(self.build_headers(false))
//             .body(data)
//             .send()?;

//         self.handler(response)
//     }

// pub fn delete(&self, endpoint: &str, listen_key: &str) ->
// Result<(String)> { let url: String = format!("{}{}", BASE_URL,
// endpoint);         let data: String = format!("listenKey={}", listen_key);

//         let client = reqwest::Client::new();
//         let response = client
//             .delete(url.as_str())
//             .headers(self.build_headers(false))
//             .body(data)
//             .send()?;

//         self.handler(response)
//     }

//     // Request must be signed
//     fn sign_request(&self, endpoint: &str, request: &str) -> String {
// let signed_key = hmac::SigningKey::new(&digest::SHA256,
// self.secret_key.as_bytes()); let signature =
// hex_encode(hmac::sign(&signed_key, request.as_bytes()).as_ref());

// let request_body: String = format!("{}&signature={}", request,
// signature); let url: String = format!("{}{}?{}", BASE_URL, endpoint,
// request_body);

//         url
//     }

// }
