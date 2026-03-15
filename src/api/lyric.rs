use anyhow::Result;
use serde_json::json;

use crate::{api::client::NeteaseApiClient, model::lyric::LyricResponse};

/// 歌词请求参数。
#[derive(Debug, Clone)]
pub struct LyricRequest {
    pub song_id: i64,
    pub cp: bool,
    pub tv: i32,
    pub lv: i32,
    pub rv: i32,
    pub kv: i32,
    pub yv: i32,
    pub ytv: i32,
    pub yrv: i32,
}

impl LyricRequest {
    /// 使用歌曲 id 创建歌词请求。
    pub fn new(song_id: i64) -> Self {
        Self {
            song_id,
            cp: false,
            tv: 0,
            lv: 0,
            rv: 0,
            kv: 0,
            yv: 0,
            ytv: 0,
            yrv: 0,
        }
    }
}

/// 获取歌曲歌词。
pub async fn get(client: &NeteaseApiClient, song_id: i64) -> Result<LyricResponse> {
    let request = LyricRequest::new(song_id);
    fetch(client, &request).await
}

/// 按结构化参数获取歌曲歌词。
pub async fn fetch(client: &NeteaseApiClient, request: &LyricRequest) -> Result<LyricResponse> {
    client
        .post_eapi(
            "/api/song/lyric/v1",
            json!({
                "id": request.song_id.to_string(),
                "cp": request.cp.to_string(),
                "tv": request.tv.to_string(),
                "lv": request.lv.to_string(),
                "rv": request.rv.to_string(),
                "kv": request.kv.to_string(),
                "yv": request.yv.to_string(),
                "ytv": request.ytv.to_string(),
                "yrv": request.yrv.to_string(),
            }),
        )
        .await
}
