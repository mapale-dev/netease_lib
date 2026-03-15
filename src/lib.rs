//! `netease_lib` provides a small, typed wrapper around Netease Cloud Music's EAPI.
//!
//! The client handles request signing/encryption and deserializes responses into Rust structs.
//!
//! ```rust,no_run
//! use netease_lib::{NeteaseApiClient, SearchQuery};
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let client = NeteaseApiClient::new()?;
//! let query = SearchQuery::new("周杰伦").with_limit(5);
//! let resp = netease_lib::api::search::artist(&client, &query).await?;
//! println!("{}", resp.code);
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod model;
mod utils;

pub use api::{
    BannerRequest, ChorusRequest, LyricRequest, MlogQuery, NeteaseApiClient, PlayerUrlRequest,
    SearchQuery,
};
