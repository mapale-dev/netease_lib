use anyhow::Result;
use serde_json::json;

use crate::{api::client::NeteaseApiClient, model::banner::BannerResponse};

/// Banner 请求参数。
#[derive(Debug, Clone)]
pub struct BannerRequest {
    pub client_type: String,
}

impl BannerRequest {
    /// 创建默认 Banner 请求。
    pub fn new() -> Self {
        Self {
            client_type: "pc".to_string(),
        }
    }

    /// 设置客户端类型。
    pub fn with_client_type(mut self, client_type: impl Into<String>) -> Self {
        self.client_type = client_type.into();
        self
    }
}

impl Default for BannerRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取 Banner 列表。
pub async fn get(client: &NeteaseApiClient, client_type: Option<&str>) -> Result<BannerResponse> {
    let request = BannerRequest::new().with_client_type(client_type.unwrap_or("pc"));
    fetch(client, &request).await
}

/// 按结构化参数获取 Banner 列表。
pub async fn fetch(client: &NeteaseApiClient, request: &BannerRequest) -> Result<BannerResponse> {
    client
        .post_eapi(
            "/api/v2/banner/get",
            json!({
                "clientType": request.client_type,
            }),
        )
        .await
}
