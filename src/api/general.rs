use api::BinanceClient;
use api::client::PublicClient;
use errors::*;
use model::{ExchangeInfo, ServerTime};
use serde_json::from_str;

pub trait GeneralApi {
    fn ping(&self) -> Result<(String)>;
    fn get_server_time(&self) -> Result<(ServerTime)>;
    fn get_exchange_info(&self) -> Result<(ExchangeInfo)>;
}

impl GeneralApi for BinanceClient {
    fn ping(&self) -> Result<(String)> {
        <PublicClient>::get(self, "/api/v1/ping", "")?;
        Ok("pong".into())
    }

    fn get_server_time(&self) -> Result<(ServerTime)> {
        let data: String = <PublicClient>::get(self, "/api/v1/time", "")?;
        let server_time: ServerTime = from_str(&data)?;
        Ok(server_time)
    }

    fn get_exchange_info(&self) -> Result<(ExchangeInfo)> {
        let data: String = <PublicClient>::get(self, "/api/v1/exchangeInfo", "")?;
        let exchange_info: ExchangeInfo = from_str(&data)?;
        Ok(exchange_info)
    }
}
