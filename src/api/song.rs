use anyhow::Result;
use serde_json::json;

use crate::{
    api::client::{NeteaseApiClient, stringify_id_list},
    model::song::ChorusResponse,
};

/// 副歌片段请求参数。
#[derive(Debug, Clone)]
pub struct ChorusRequest {
    pub ids: Vec<i64>,
}

impl ChorusRequest {
    /// 使用歌曲 id 列表创建副歌请求。
    pub fn new(ids: impl Into<Vec<i64>>) -> Self {
        Self { ids: ids.into() }
    }
}

/// 获取多首歌曲的副歌片段。
pub async fn chorus(client: &NeteaseApiClient, ids: &[i64]) -> Result<ChorusResponse> {
    let request = ChorusRequest::new(ids.to_vec());
    fetch_chorus(client, &request).await
}

/// 按结构化参数获取多首歌曲的副歌片段。
pub async fn fetch_chorus(client: &NeteaseApiClient, request: &ChorusRequest) -> Result<ChorusResponse> {
    client
        .post_eapi(
            "/api/song/chorus",
            json!({
                "ids": stringify_id_list(&request.ids),
            }),
        )
        .await
}

/// 歌曲增强接口。
pub mod enhance {
    /// 播放器相关接口。
    pub mod player {
        use anyhow::Result;
        use serde_json::json;

        use crate::{
            api::client::{NeteaseApiClient, stringify_id_list},
            model::song::PlayerUrlResponse,
        };

        /// 播放地址请求参数。
        #[derive(Debug, Clone)]
        pub struct PlayerUrlRequest {
            pub ids: Vec<i64>,
            pub level: String,
            pub encode_type: String,
            pub immerse_type: String,
            pub trial_mode: String,
        }

        impl PlayerUrlRequest {
            /// 使用歌曲 id 列表创建播放地址请求。
            pub fn new(ids: impl Into<Vec<i64>>) -> Self {
                Self {
                    ids: ids.into(),
                    level: "exhigh".to_string(),
                    encode_type: "aac".to_string(),
                    immerse_type: "c51".to_string(),
                    trial_mode: "-1".to_string(),
                }
            }

            /// 设置音质等级。
            pub fn with_level(mut self, level: impl Into<String>) -> Self {
                self.level = level.into();
                self
            }

            /// 设置编码格式。
            pub fn with_encode_type(mut self, encode_type: impl Into<String>) -> Self {
                self.encode_type = encode_type.into();
                self
            }

            /// 设置沉浸音类型。
            pub fn with_immerse_type(mut self, immerse_type: impl Into<String>) -> Self {
                self.immerse_type = immerse_type.into();
                self
            }
        }

        /// 获取歌曲播放地址。
        pub async fn url_v1(
            client: &NeteaseApiClient,
            ids: &[i64],
            level: Option<&str>,
            encode_type: Option<&str>,
            immerse_type: Option<&str>,
        ) -> Result<PlayerUrlResponse> {
            let mut request = PlayerUrlRequest::new(ids.to_vec());
            if let Some(level) = level {
                request = request.with_level(level);
            }
            if let Some(encode_type) = encode_type {
                request = request.with_encode_type(encode_type);
            }
            if let Some(immerse_type) = immerse_type {
                request = request.with_immerse_type(immerse_type);
            }

            fetch_url_v1(client, &request).await
        }

        /// 按结构化参数获取歌曲播放地址。
        pub async fn fetch_url_v1(
            client: &NeteaseApiClient,
            request: &PlayerUrlRequest,
        ) -> Result<PlayerUrlResponse> {
            client
                .post_eapi(
                    "/api/song/enhance/player/url/v1",
                    json!({
                        "ids": stringify_id_list(&request.ids),
                        "level": request.level,
                        "immerseType": request.immerse_type,
                        "encodeType": request.encode_type,
                        "trialMode": request.trial_mode,
                    }),
                )
                .await
        }
    }
}
