use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 歌词贡献用户信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LyricUser {
    pub id: i64,
    pub status: i32,
    pub demand: i32,
    #[serde(rename = "userid")]
    pub user_id: i64,
    pub nickname: String,
    pub uptime: i64,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 原文歌词信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Lrc {
    pub version: i32,
    pub lyric: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 翻译歌词信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TLyric {
    pub version: i32,
    pub lyric: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 罗马音歌词信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RomaLrc {
    pub version: i32,
    pub lyric: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 歌词接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LyricResponse {
    pub sgc: bool,
    pub sfy: bool,
    pub qfy: bool,
    #[serde(rename = "lyricUser", default)]
    pub lyric_user: Option<LyricUser>,
    #[serde(default)]
    pub lrc: Option<Lrc>,
    #[serde(rename = "tlyric", default)]
    pub t_lyric: Option<TLyric>,
    #[serde(rename = "romalrc", default)]
    pub roma_lrc: Option<RomaLrc>,
    pub code: i32,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
