// use util::{build_signed_request, UrlParameters};
// use model::{AccountInformation, Balance, Order, OrderCanceled, TradeHistory, Transaction};
// use client::Client;
// use errors::*;
// use serde_json::from_str;

// static ORDER_TYPE_LIMIT: &'static str = "LIMIT";
// static ORDER_TYPE_MARKET: &'static str = "MARKET";
// static ORDER_SIDE_BUY: &'static str = "BUY";
// static ORDER_SIDE_SELL: &'static str = "SELL";
// static TIME_IN_FORCE_GTC: &'static str = "GTC";

// static API_V3_ORDER: &'static str = "/api/v3/order";

// /// Binance Account Client struct
// #[derive(Clone, Debug)]
// pub struct Account {
//     /// Binance Client.
//     pub client: Client,
//     /// Number of milliseconds a signed request is valid for.
//     pub recv_window: Option<u64>,
// }

// impl Account {
//     /// Gets current account information.
//     /// 
//     /// Endpoint Security Type: USER_DATA
//     /// 
//     /// Weight: 20
//     pub fn get_account(&self) -> Result<(AccountInformation)> {
//         let data = self.client.get_signed(
//             "/api/v3/account",
//             &build_signed_request(UrlParameters::default(), self.recv_window),
//         )?;
//         let account_info: AccountInformation = from_str(data.as_str()).unwrap();

//         Ok(account_info)
//     }

//     // Balance for ONE Asset
//     pub fn get_balance(&self, asset: &str) -> Result<(Balance)> {
//         match self.get_account() {
//             Ok(account) => {
//                 for balance in account.balances {
//                     if balance.asset == asset {
//                         return Ok(balance);
//                     }
//                 }
//                 bail!("Asset not found");
//             }
//             Err(e) => Err(e),
//         }
//     }

//     // Current open orders for ONE symbol
//     pub fn get_open_orders(&self, symbol: &str) -> Result<(Vec<Order>)> {
//         let parameters = UrlParameters::new("symbol", symbol);
//         let data = self.client.get_signed(
//             "/api/v3/openOrders",
//             &build_signed_request(parameters, self.recv_window),
//         )?;
//         let order: Vec<Order> = from_str(data.as_str()).unwrap();

//         Ok(order)
//     }

//     // Check an order's status
//     pub fn order_status(&self, symbol: &str, order_id: u32) -> Result<(Order)> {
//         let mut parameters = UrlParameters::new("symbol", symbol);
//         parameters.append("orderId", &order_id.to_string());
//         let data = self.client.get_signed(
//             API_V3_ORDER,
//             &build_signed_request(parameters, self.recv_window),
//         )?;
//         let order: Order = from_str(data.as_str()).unwrap();

//         Ok(order)
//     }

//     // Place a LIMIT order - BUY
//     pub fn limit_buy(&self, symbol: &str, qty: u32, price: f64) -> Result<(Transaction)> {
//         let order = self.build_order(
//             symbol,
//             qty,
//             price,
//             ORDER_SIDE_BUY,
//             ORDER_TYPE_LIMIT,
//             TIME_IN_FORCE_GTC,
//         );
//         let data = self.client
//             .post_signed(API_V3_ORDER, &build_signed_request(order, self.recv_window))?;
//         let transaction: Transaction = from_str(data.as_str()).unwrap();

//         Ok(transaction)
//     }

//     // Place a LIMIT order - SELL
//     pub fn limit_sell(&self, symbol: &str, qty: u32, price: f64) -> Result<(Transaction)> {
//         let order = self.build_order(
//             symbol,
//             qty,
//             price,
//             ORDER_SIDE_SELL,
//             ORDER_TYPE_LIMIT,
//             TIME_IN_FORCE_GTC,
//         );
//         let data = self.client
//             .post_signed(API_V3_ORDER, &build_signed_request(order, self.recv_window))?;
//         let transaction: Transaction = from_str(data.as_str()).unwrap();

//         Ok(transaction)
//     }

//     // Place a MARKET order - BUY
//     pub fn market_buy(&self, symbol: &str, qty: u32) -> Result<(Transaction)> {
//         let order = self.build_order(
//             symbol,
//             qty,
//             0.0,
//             ORDER_SIDE_BUY,
//             ORDER_TYPE_MARKET,
//             TIME_IN_FORCE_GTC,
//         );
//         let data = self.client
//             .post_signed(API_V3_ORDER, &build_signed_request(order, self.recv_window))?;
//         let transaction: Transaction = from_str(data.as_str()).unwrap();

//         Ok(transaction)
//     }

//     // Place a MARKET order - SELL
//     pub fn market_sell(&self, symbol: &str, qty: u32) -> Result<(Transaction)> {
//         let order = self.build_order(
//             symbol,
//             qty,
//             0.0,
//             ORDER_SIDE_SELL,
//             ORDER_TYPE_MARKET,
//             TIME_IN_FORCE_GTC,
//         );
//         let data = self.client
//             .post_signed(API_V3_ORDER, &build_signed_request(order, self.recv_window))?;
//         let transaction: Transaction = from_str(data.as_str()).unwrap();

//         Ok(transaction)
//     }

//     // Check an order's status
//     pub fn cancel_order(&self, symbol: &str, order_id: u32) -> Result<(OrderCanceled)> {
//         let mut parameters = UrlParameters::new("symbol", symbol);
//         parameters.append("orderId", &order_id.to_string());
//         let data = self.client.delete_signed(
//             API_V3_ORDER,
//             &build_signed_request(parameters, self.recv_window),
//         )?;
//         let order_canceled: OrderCanceled = from_str(data.as_str()).unwrap();

//         Ok(order_canceled)
//     }

//     // Trade history
//     pub fn trade_history(&self, symbol: &str) -> Result<(Vec<TradeHistory>)> {
//         let parameters = UrlParameters::new("symbol", symbol);
//         let data = self.client.get_signed(
//             "/api/v3/myTrades",
//             &build_signed_request(parameters, self.recv_window),
//         )?;
//         let trade_history: Vec<TradeHistory> = from_str(data.as_str()).unwrap();

//         Ok(trade_history)
//     }

//     fn build_order(
//         &self,
//         symbol: &str,
//         qty: u32,
//         price: f64,
//         order_side: &str,
//         order_type: &str,
//         time_in_force: &str,
//     ) -> UrlParameters {
//         let mut parameters = UrlParameters::new("symbol", symbol);
//         parameters.append("side", order_side);
//         parameters.append("type", order_type);
//         parameters.append("quantity", &qty.to_string());

//         if price != 0.0 {
//             parameters.append("price", &price.to_string());
//             parameters.append("timeInForce", &time_in_force.to_string());
//         }

//         parameters
//     }
// }
