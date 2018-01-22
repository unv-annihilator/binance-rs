use errors::ErrorKind;
use serde::{de, Deserializer};
use serde::de::Visitor;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: Timestamp,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub maker_commission: f64,
    pub taker_commission: f64,
    pub buyer_commission: f64,
    pub seller_commission: f64,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub update_time: Timestamp,
    pub balances: Vec<Balance>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    pub free: f64,
    pub locked: f64,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Asset(pub String); // BTC

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Symbol(pub String); // ETHBTC

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: Asset,
    pub order_id: u32,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    pub stop_price: String,
    pub iceberg_qty: String,
    pub time: u64,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceled {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: u32,
    pub client_order_id: String,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub symbol: String,
    pub order_id: u32,
    pub client_order_id: String,
    pub transact_time: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Bids {
    price: String,
    qty: String,

    // Never serialized.
    #[serde(skip_serializing)]
    ignore: Vec<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Asks {
    price: String,
    qty: String,

    // Never serialized.
    #[serde(skip_serializing)]
    ignore: Vec<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStream {
    pub listen_key: String,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Success {}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct SymbolPrice {
    pub symbol: String,
    pub price: String,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct AllPrices(Vec<SymbolPrice>);

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickers {
    AllBookTickers(Vec<Tickers>),
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
    pub symbol: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub id: u32,
    pub price: String,
    pub qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerStats {
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub prev_close_price: String,
    pub last_price: String,
    pub bid_price: String,
    pub ask_price: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub volume: String,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct AllTickerStats(Vec<TickerStats>);

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub filter_type: String,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeSymbol {
    pub symbol: Symbol,
    pub status: String,
    pub base_asset: Asset,
    pub base_asset_precision: u64,
    pub quote_asset: Asset,
    pub quote_precision: u64,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub filters: Vec<Filter>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub limit: u64,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub timezone: String,
    pub server_time: Timestamp,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<Filter>,
    pub symbols: Vec<ExchangeSymbol>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    #[serde(deserialize_with = "f64_from_str")]
    pub price: f64,
    #[serde(deserialize_with = "f64_from_str")]
    pub qty: f64,
    pub time: Timestamp,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trades(Vec<Trade>);

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedTrade {
    #[serde(rename = "a")]
    pub trade_id: u64,
    #[serde(deserialize_with = "f64_from_str")]
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(deserialize_with = "f64_from_str")]
    #[serde(rename = "q")]
    pub quantity: f64,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub timestamp: Timestamp,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M")]
    pub is_best_match: bool,
    // Trade was best price match
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedTrades(Vec<AggregatedTrade>);

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    m: u32,
    t: u32,
    b: u32,
    s: u32,
    #[serde(rename = "T")]
    t_ignore: bool,
    #[serde(rename = "W")]
    w_ignore: bool,
    #[serde(rename = "D")]
    d_ignore: bool,
    #[serde(rename = "B")]
    pub balance: Vec<EventBalance>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventBalance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f")]
    pub free: String,
    #[serde(rename = "l")]
    pub locked: String,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderTradeEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub new_client_order_id: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "o")]
    pub order_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "q")]
    pub qty: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(skip_serializing, rename = "P")]
    pub p_ignore: String,
    #[serde(skip_serializing, rename = "F")]
    pub f_ignore: String,
    #[serde(skip_serializing)]
    pub g: i32,
    #[serde(skip_serializing, rename = "C")]
    pub c_ignore: Option<String>,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "X")]
    pub order_status: String,
    #[serde(rename = "r")]
    pub order_reject_reason: String,
    #[serde(rename = "i")]
    pub order_id: u32,
    #[serde(rename = "l")]
    pub qty_last_filled_trade: String,
    #[serde(rename = "z")]
    pub accumulated_qty_filled_trades: String,
    #[serde(rename = "L")]
    pub price_last_filled_trade: String,
    #[serde(rename = "n")]
    pub commission: String,
    #[serde(skip_serializing, rename = "N")]
    pub asset_commisioned: Option<String>,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "t")]
    pub trade_id: i32,
    #[serde(skip_serializing, rename = "I")]
    pub i_ignore: u32,
    #[serde(skip_serializing)]
    pub w: bool,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradesEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub aggregated_trade_id: u32,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub qty: String,
    #[serde(rename = "f")]
    pub first_break_trade_id: u32,
    #[serde(rename = "l")]
    pub last_break_trade_id: u32,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum ChartInterval {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    OneDay,
    ThreeDays,
    OneWeek,
    OneMonth,
}

impl ChartInterval {
    pub fn as_str(&self) -> &str {
        match *self {
            ChartInterval::OneMinute => "1m",
            ChartInterval::ThreeMinutes => "3m",
            ChartInterval::FiveMinutes => "5m",
            ChartInterval::FifteenMinutes => "15m",
            ChartInterval::ThirtyMinutes => "30m",
            ChartInterval::OneHour => "1h",
            ChartInterval::TwoHours => "2h",
            ChartInterval::FourHours => "4h",
            ChartInterval::SixHours => "6h",
            ChartInterval::EightHours => "8h",
            ChartInterval::TwelveHours => "12h",
            ChartInterval::OneDay => "1d",
            ChartInterval::ThreeDays => "3d",
            ChartInterval::OneWeek => "1w",
            ChartInterval::OneMonth => "1M",
        }
    }
}

impl FromStr for ChartInterval {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1m" => Ok(ChartInterval::OneMinute),
            "3m" => Ok(ChartInterval::ThreeMinutes),
            "5m" => Ok(ChartInterval::FiveMinutes),
            "15m" => Ok(ChartInterval::FifteenMinutes),
            "30m" => Ok(ChartInterval::ThirtyMinutes),
            "1h" => Ok(ChartInterval::OneHour),
            "2h" => Ok(ChartInterval::TwoHours),
            "4h" => Ok(ChartInterval::FourHours),
            "6h" => Ok(ChartInterval::SixHours),
            "8h" => Ok(ChartInterval::EightHours),
            "12h" => Ok(ChartInterval::TwelveHours),
            "1d" => Ok(ChartInterval::OneDay),
            "3d" => Ok(ChartInterval::ThreeDays),
            "1w" => Ok(ChartInterval::OneWeek),
            "1M" => Ok(ChartInterval::OneMonth),
            _ => Err(ErrorKind::ParseIntervalError),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: Kline,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: ChartInterval,
    #[serde(rename = "f")]
    pub first_trade_id: i32,
    #[serde(rename = "L")]
    pub last_trade_id: i32,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "n")]
    pub number_of_trades: i32,
    #[serde(rename = "x")]
    pub is_final_bar: bool,
    #[serde(rename = "q")]
    pub quote_volume: String,
    #[serde(rename = "V")]
    pub active_buy_volume: String,
    #[serde(rename = "Q")]
    pub active_volume_buy_quote: String,
    #[serde(skip_serializing, rename = "B")]
    pub ignore_me: String,
}

// Fancy str to F64 deserialization
// More info:
//  https://users.rust-lang.org/t/serde-deserialize-string-field-in-json-to-a-different-type/12942/11
struct F64FromStr;

impl<'de> Visitor<'de> for F64FromStr {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing a f64 number")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        f64::from_str(s).map_err(de::Error::custom)
    }
}

// Function to deserialize from a str into a f64
fn f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(F64FromStr)
}
