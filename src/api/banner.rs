use anyhow::Result;
use serde_json::json;

use crate::{api::client::NeteaseApiClient, model::banner::BannerResponse};

/// 获取 Banner 列表。
pub async fn get(client: &NeteaseApiClient, client_type: Option<&str>) -> Result<BannerResponse> {
    client
        .post_eapi(
            "/api/v2/banner/get",
            json!({
                "clientType": client_type.unwrap_or("pc"),
            }),
        )
        .await
}
