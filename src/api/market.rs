use api::BinanceClient;
use api::client::{ApiClient, PublicClient};
use api::util::UrlParameters;
use errors::*;
use model::*;
use serde_json::from_str;

pub trait MarketApi {
    // /api/v1/depth
    fn get_depth(&self, symbol: &str, limit: Option<i32>) -> Result<(OrderBook)>;
    // /api/v1/trades
    fn get_trades(&self, symbol: &str) -> Result<(Trades)>;
    // /api/v1/historicalTrades
    fn get_historical_trades(&self, symbol: &str, from_id: Option<u64>) -> Result<(Trades)>;
    // /api/v1/aggTrades
    fn get_agg_trades(
        &self,
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<Timestamp>,
        end_time: Option<Timestamp>,
    ) -> Result<(AggregatedTrades)>;
    // /api/v1/klines  | Pending parse/deserialization setup
    // fn get_klines(&self, symbol: &str, interval: ChartInterval, start_time:
    // Option<Timestamp>, end_time: Option<Timestamp>) -> Result<(Vec<KLine)>
    // /api/v1/ticker/24hr
    fn get_24h_ticker_stats(&self, symbol: &str) -> Result<(TickerStats)>;
    fn get_all_24h_ticker_stats(&self) -> Result<(AllTickerStats)>;
    // /api/v3/ticker/price
    fn get_ticker_price(&self, symbol: &str) -> Result<(SymbolPrice)>;
    fn get_all_ticker_prices(&self) -> Result<(AllPrices)>;
}

impl MarketApi for BinanceClient {
    fn get_depth(&self, symbol: &str, limit: Option<i32>) -> Result<(OrderBook)> {
        let mut parameters = UrlParameters::new(&"symbol", &symbol);
        if let Some(custom_limit) = limit {
            match custom_limit {
                5 | 10 | 20 | 50 | 100 | 500 | 1000 => parameters.append(&"limit", &custom_limit),
                _ => bail!("ERROR! Invalid Limit Parameter!"),
            }
        }
        let data = <PublicClient>::get(self, "/api/v1/depth", &parameters.to_string())?;
        let order_book: OrderBook = from_str(&data)?;
        Ok(order_book)
    }
    /// Get recent trades (up to last 500)
    fn get_trades(&self, symbol: &str) -> Result<(Trades)> {
        let data = <PublicClient>::get(
            self,
            "/api/v1/trades",
            &UrlParameters::new(&"symbol", &symbol).to_string(),
        )?;
        let trades: Trades = from_str(&data)?;
        Ok(trades)
    }
    /// Get older trades (up to 500 at a time)
    fn get_historical_trades(&self, symbol: &str, from_id: Option<u64>) -> Result<(Trades)> {
        let mut parameters = UrlParameters::new(&"symbol", &symbol);
        if let Some(id) = from_id {
            parameters.append(&"fromId", &id);
        }
        let data =
            <ApiClient>::get_with_key(self, "/api/v1/historicalTrades", &parameters.to_string())?;
        let trades: Trades = from_str(&data)?;
        Ok(trades)
    }
    /// Get compressed, aggregate trades. Trades that fill at the time, from the same order, with
    /// the same price will have the quantity aggregated. If both startTime and endTime are
    /// sent, limit should not be sent AND the distance between startTime and endTime must be less
    /// than 24 hours. If fromId, startTime, and endTime are not sent, the most recent
    /// aggregate trades will be returned.
    fn get_agg_trades(
        &self,
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<Timestamp>,
        end_time: Option<Timestamp>,
    ) -> Result<(AggregatedTrades)> {
        let mut parameters = UrlParameters::new(&"symbol", &symbol);
        if let Some(id) = from_id {
            parameters.append(&"fromId", &id);
        }
        if let Some(start) = start_time {
            parameters.append(&"startTime", &start);
        }
        if let Some(end) = end_time {
            parameters.append(&"endTime", &end);
        }
        let data = <PublicClient>::get(self, "/api/v1/aggTrades", &parameters.to_string())?;
        let trades: AggregatedTrades = from_str(&data)?;
        Ok(trades)
    }
    /// 24 hour price change statistics for symbol.
    fn get_24h_ticker_stats(&self, symbol: &str) -> Result<(TickerStats)> {
        let data = <PublicClient>::get(
            self,
            "/api/v1/ticker/24hr",
            &UrlParameters::new(&"symbol", &symbol).to_string(),
        )?;
        let stats: TickerStats = from_str(&data)?;
        Ok(stats)
    }
    /// 24 hour price change statistics for all trading symbols.
    fn get_all_24h_ticker_stats(&self) -> Result<(AllTickerStats)> {
        let data = <PublicClient>::get(self, "/api/v1/ticker/24hr", "")?;
        let stats: AllTickerStats = from_str(&data)?;
        Ok(stats)
    }
    /// Latest price for a symbol.
    fn get_ticker_price(&self, symbol: &str) -> Result<(SymbolPrice)> {
        let data = <PublicClient>::get(
            self,
            "/api/v3/ticker/price",
            &UrlParameters::new(&"symbol", &symbol).to_string(),
        )?;
        let price: SymbolPrice = from_str(&data)?;
        Ok(price)
    }
    /// Latest price for symbols.
    fn get_all_ticker_prices(&self) -> Result<(AllPrices)> {
        let data = <PublicClient>::get(self, "/api/v3/ticker/price", "")?;
        let prices: AllPrices = from_str(&data)?;
        Ok(prices)
    }
}
