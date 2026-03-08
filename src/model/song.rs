use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::common::FreeTrialPrivilege;

/// 副歌片段信息，用于歌曲高潮识别接口。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChorusSegment {
    pub id: i64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "ugcLocked", default)]
    pub ugc_locked: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 副歌识别接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChorusResponse {
    pub code: i32,
    #[serde(default)]
    pub chorus: Vec<ChorusSegment>,
    #[serde(default)]
    pub data: Vec<ChorusSegment>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 单首歌曲播放地址信息，包含播放链接、音质和权限字段。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerUrlItem {
    pub id: i64,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub br: Option<i32>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub md5: Option<String>,
    pub code: i32,
    #[serde(default)]
    pub expi: Option<i32>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub gain: Option<f64>,
    #[serde(default)]
    pub peak: Option<f64>,
    #[serde(rename = "closedGain", default)]
    pub closed_gain: Option<f64>,
    #[serde(rename = "closedPeak", default)]
    pub closed_peak: Option<f64>,
    #[serde(default)]
    pub fee: Option<i32>,
    #[serde(default)]
    pub uf: Option<Value>,
    #[serde(default)]
    pub payed: Option<i32>,
    #[serde(default)]
    pub flag: Option<i64>,
    #[serde(rename = "canExtend", default)]
    pub can_extend: Option<bool>,
    #[serde(rename = "freeTrialInfo", default)]
    pub free_trial_info: Option<Value>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(rename = "encodeType", default)]
    pub encode_type: Option<String>,
    #[serde(rename = "channelLayout", default)]
    pub channel_layout: Option<String>,
    #[serde(rename = "freeTrialPrivilege", default)]
    pub free_trial_privilege: Option<FreeTrialPrivilege>,
    #[serde(rename = "freeTimeTrialPrivilege", default)]
    pub free_time_trial_privilege: Option<FreeTrialPrivilege>,
    #[serde(rename = "urlSource", default)]
    pub url_source: Option<i32>,
    #[serde(rename = "rightSource", default)]
    pub right_source: Option<i32>,
    #[serde(rename = "podcastCtrp", default)]
    pub podcast_ctrp: Option<String>,
    #[serde(rename = "effectTypes", default)]
    pub effect_types: Option<Value>,
    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(rename = "levelConfuse", default)]
    pub level_confuse: Option<Value>,
    #[serde(rename = "musicId", default)]
    pub music_id: Option<String>,
    #[serde(default)]
    pub accompany: Option<Value>,
    #[serde(default)]
    pub sr: Option<i32>,
    #[serde(rename = "auEff", default)]
    pub au_eff: Option<Value>,
    #[serde(rename = "immerseType", default)]
    pub immerse_type: Option<String>,
    #[serde(rename = "beatType", default)]
    pub beat_type: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 播放地址接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerUrlResponse {
    #[serde(default)]
    pub data: Vec<PlayerUrlItem>,
    pub code: i32,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
