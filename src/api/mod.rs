// API Utils
mod util;
// HTTP Client
mod client;
// General API Endpoint
mod general;
// Market API Endpoint
mod market;

// Alias Client into BinanceClient
pub type BinanceClient = client::Client;
// Export GeneralApi to make it available off API
pub use self::general::GeneralApi;
// Export MarketApi to make it available off API
pub use self::market::MarketApi;

// Implement BinanceClient new
impl BinanceClient {
    pub fn new(api_key: &str, secret_key: &str) -> BinanceClient {
        BinanceClient {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
        }
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = api_key.into();
    }

    pub fn set_secret_key(&mut self, secret_key: &str) {
        self.secret_key = secret_key.into();
    }
}
