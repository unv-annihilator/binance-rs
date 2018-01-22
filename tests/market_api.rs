extern crate binance;
extern crate config;

use binance::api::BinanceClient;
use binance::api::MarketApi;
use binance::model::{AggregatedTrades, AllPrices, AllTickerStats, OrderBook, SymbolPrice,
                     TickerStats, Trades};

// Market API Tests
#[test]
fn get_depth_ok() {
    let client = BinanceClient::default();
    let _order_book: OrderBook = client.get_depth("ETHBTC", None).unwrap();
    assert!(true);
}

#[test]
fn get_trades_ok() {
    let client = BinanceClient::default();
    let _trades: Trades = client.get_trades("ETHBTC").unwrap();
    assert!(true);
}

#[test]
fn get_historical_trades_ok() {
    let mut client = BinanceClient::default();
    // Load API key from config
    let mut config_file = config::Config::default();
    config_file
        .merge(config::File::with_name("config"))
        .unwrap();
    let apikey = config_file.get_str("APIKEY").unwrap();
    client.set_api_key(&apikey);
    let _trades: Trades = client.get_historical_trades("XVGBTC", None).unwrap();
    assert!(true);
}

#[test]
fn get_agg_trades_ok() {
    let client = BinanceClient::default();
    let _agg_trades: AggregatedTrades = client.get_agg_trades("ETHBTC", None, None, None).unwrap();
    assert!(true);
}

#[test]
fn get_24hr_ticker_stats_ok() {
    let client = BinanceClient::default();
    let _stats: TickerStats = client.get_24h_ticker_stats("XVGBTC").unwrap();
    assert!(true);
}

#[test]
fn get_all_24hr_ticker_stats_ok() {
    let client = BinanceClient::default();
    let _all_stats: AllTickerStats = client.get_all_24h_ticker_stats().unwrap();
    assert!(true);
}

#[test]
fn get_ticker_price_ok() {
    let client = BinanceClient::default();
    let _price: SymbolPrice = client.get_ticker_price("ETHBTC").unwrap();
    assert!(true);
}

#[test]
fn get_all_ticker_prices_ok() {
    let client = BinanceClient::default();
    let _prices: AllPrices = client.get_all_ticker_prices().unwrap();
    assert!(true);
}
