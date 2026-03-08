use anyhow::Result;
use serde_json::json;

use crate::{api::client::NeteaseApiClient, model::lyric::LyricResponse};

/// 获取歌曲歌词。
pub async fn get(client: &NeteaseApiClient, song_id: i64) -> Result<LyricResponse> {
    client
        .post_eapi(
            "/api/song/lyric/v1",
            json!({
                "id": song_id.to_string(),
                "cp": "false",
                "tv": "0",
                "lv": "0",
                "rv": "0",
                "kv": "0",
                "yv": "0",
                "ytv": "0",
                "yrv": "0",
            }),
        )
        .await
}
