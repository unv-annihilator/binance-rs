extern crate binance;

use binance::api::*;

fn main() {
    general();
    market();
}

fn general() {
    // General API doesn't require ApiKey or SecretKey
    // So we can just make a default client
    let client = BinanceClient::default();

    let ping_response = client.ping();
    match ping_response {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let time_response = client.get_server_time();
    match time_response {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }

    let exchange_info_response = client.get_exchange_info();
    match exchange_info_response {
        Ok(answer) => println!("Exchange Response: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

fn market() {
    // Not everything requires an api key!
    let mut client = BinanceClient::default();

    let depth_response = client.get_depth("XVGBTC", None);
    match depth_response {
        Ok(answer) => println!("Depth: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    // Get Recent Trades (up to last 500)
    let trades_response = client.get_trades("XVGBTC");
    match trades_response {
        Ok(answer) => println!("Trades: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    // 24 hour price change statistics.
    let ticker_stats = client.get_24h_ticker_stats("XVGBTC");
    match ticker_stats {
        Ok(answer) => println!("XVGBTC 24hr Stats: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    // 24 hour price change statistics for ALL trading symbols
    let all_stats = client.get_all_24h_ticker_stats();
    match all_stats {
        Ok(answer) => println!("All 24hr Stats: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    // Latest price for a symbol.
    let ticker_price = client.get_ticker_price("ETHBTC");
    match ticker_price {
        Ok(answer) => println!("ETHBTC Price: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    // Latest price for symbols.
    let ticker_prices = client.get_all_ticker_prices();
    match ticker_prices {
        Ok(answer) => println!("All Ticker Prices: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // But some API calls do
    client.set_api_key("API_KEY");
    // Get older trades (up to 500 at a time)
    let hist_trades_response = client.get_historical_trades("XVGBTC", None);
    match hist_trades_response {
        Ok(answer) => println!("Hist Trades: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}
