extern crate binance;

use binance::api::BinanceClient;
use binance::api::GeneralApi;
use binance::model::{ExchangeInfo, ServerTime};

// General API Tests
#[test]
fn server_ping_ok() {
    let client = BinanceClient::default();
    let response = client.ping();
    // Unwrap response to panic on error
    assert_eq!(response.unwrap(), "pong");
    // Should always return pong if successful
}

#[test]
fn server_time_ok() {
    let client = BinanceClient::default();
    let _server_time: ServerTime = client.get_server_time().unwrap();
    // Not checking for a server response, just lack of error in the call
    assert!(true);
    // If it asserts to True we received a valid response/parsed correctly
}

#[test]
fn get_exchange_ok() {
    let client = BinanceClient::default();
    let _exchange_info: ExchangeInfo = client.get_exchange_info().unwrap();
    // Not checking for a server response, just lack of error in the call
    assert!(true);
    // If it gets to the assert it was able to parse correctly
}
