// use model::{Success, UserDataStream};
// use client::Client;
// use errors::*;
// use serde_json::from_str;

// static USER_DATA_STREAM: &'static str = "/api/v1/userDataStream";

// #[derive(Clone, Debug)]
// pub struct UserStream {
//     pub client: Client,
//     pub recv_window: u64,
// }

// impl UserStream {
//     // User Stream
//     pub fn start(&self) -> Result<(UserDataStream)> {
//         let data = self.client.post(USER_DATA_STREAM)?;
//         let user_data_stream: UserDataStream = from_str(data.as_str()).unwrap();

//         Ok(user_data_stream)
//     }

//     // Current open orders on a symbol
//     pub fn keep_alive(&self, listen_key: &str) -> Result<(Success)> {
//         let data = self.client.put(USER_DATA_STREAM, listen_key)?;

//         let success: Success = from_str(data.as_str()).unwrap();

//         Ok(success)
//     }

//     pub fn close(&self, listen_key: &str) -> Result<(Success)> {
//         let data = self.client.put(USER_DATA_STREAM, listen_key)?;

//         let success: Success = from_str(data.as_str()).unwrap();

//         Ok(success)
//     }
// }
