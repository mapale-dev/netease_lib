use anyhow::Result;
use serde_json::json;

use crate::{
    api::client::{NeteaseApiClient, stringify_id_list},
    model::song::ChorusResponse,
};

/// 获取多首歌曲的副歌片段。
pub async fn chorus(client: &NeteaseApiClient, ids: &[i64]) -> Result<ChorusResponse> {
    client
        .post_eapi(
            "/api/song/chorus",
            json!({
                "ids": stringify_id_list(ids),
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

        /// 获取歌曲播放地址。
        pub async fn url_v1(
            client: &NeteaseApiClient,
            ids: &[i64],
            level: Option<&str>,
            encode_type: Option<&str>,
            immerse_type: Option<&str>,
        ) -> Result<PlayerUrlResponse> {
            client
                .post_eapi(
                    "/api/song/enhance/player/url/v1",
                    json!({
                        "ids": stringify_id_list(ids),
                        "level": level.unwrap_or("exhigh"),
                        "immerseType": immerse_type.unwrap_or("c51"),
                        "encodeType": encode_type.unwrap_or("aac"),
                        "trialMode": "-1",
                    }),
                )
                .await
        }
    }
}
