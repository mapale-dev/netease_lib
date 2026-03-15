/// Banner 接口封装。
pub mod banner;
/// 通用 HTTP 客户端与 EAPI 请求封装。
pub mod client;
/// 歌词接口封装。
pub mod lyric;
/// 搜索接口封装。
pub mod search;
/// 歌曲接口封装。
pub mod song;

pub use client::NeteaseApiClient;
pub use banner::BannerRequest;
pub use lyric::LyricRequest;
pub use search::{MlogQuery, SearchQuery};
pub use song::ChorusRequest;
pub use song::enhance::player::PlayerUrlRequest;
