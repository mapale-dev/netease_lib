use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Banner 规则容器，对应返回中的规则列表。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BannerRuleContainer {
    #[serde(default)]
    pub rules: Vec<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 单个 Banner 项，包含跳转目标和展示图片等信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BannerItem {
    #[serde(rename = "targetId")]
    pub target_id: i64,
    #[serde(rename = "bigImageUrl", default)]
    pub big_image_url: Option<String>,
    #[serde(rename = "imageUrl", default)]
    pub image_url: Option<String>,
    #[serde(rename = "targetType", default)]
    pub target_type: Option<i32>,
    #[serde(rename = "typeTitle", default)]
    pub type_title: Option<String>,
    #[serde(default)]
    pub s_ctrp: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Banner 接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BannerResponse {
    pub code: i32,
    #[serde(default)]
    pub trp: Option<BannerRuleContainer>,
    #[serde(default)]
    pub banners: Vec<BannerItem>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
