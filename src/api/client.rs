use anyhow::{Context, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::utils::{cline_sign::get_local_mac, crypto::netease_req_encrypt};

/// 默认网易云接口根地址。
pub const DEFAULT_BASE_URL: &str = "https://interface.music.163.com";

/// 网易云 EAPI 客户端，负责构建请求头、加密参数并解析响应。
#[derive(Clone, Debug)]
pub struct NeteaseApiClient {
    client: Client,
    base_url: String,
    app_version: String,
    os: String,
    os_version: String,
    device_id: String,
    client_sign: String,
}

impl NeteaseApiClient {
    /// 创建一个使用默认配置的客户端。
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("NeteaseMusicDesktop/3.1.28.205001")
            .build()
            .context("failed to build reqwest client")?;

        let mac = get_local_mac().unwrap_or_default();
        let normalized_mac = normalize_mac(&mac);
        let device_id = build_device_id(&normalized_mac);
        let client_sign = build_client_sign(&normalized_mac);

        Ok(Self {
            client,
            base_url: DEFAULT_BASE_URL.to_string(),
            app_version: "3.1.28.205001".to_string(),
            os: "pc".to_string(),
            os_version: detect_os_version(),
            device_id,
            client_sign,
        })
    }

    /// 使用自定义基础地址创建客户端。
    pub fn with_base_url(base_url: impl Into<String>) -> Result<Self> {
        let mut client = Self::new()?;
        client.base_url = base_url.into();
        Ok(client)
    }

    /// 发送 EAPI 请求并解析为指定响应类型。
    pub async fn post_eapi<R>(&self, api_path: &str, payload: Value) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let payload = self.finalize_payload(payload);
        let payload_text = serde_json::to_string(&payload).context("failed to serialize payload")?;
        let params = netease_req_encrypt(api_path, &payload_text).context("failed to encrypt payload")?;
        let endpoint = format!("{}{}", self.base_url, to_eapi_path(api_path));

        let response_text = self
            .client
            .post(endpoint)
            .header("referer", "https://music.163.com")
            .header("origin", "https://music.163.com")
            .header("x-requested-with", "XMLHttpRequest")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(format!("params={params}"))
            .send()
            .await
            .context("request failed")?
            .error_for_status()
            .context("request returned error status")?
            .text()
            .await
            .context("failed to read response body")?;

        serde_json::from_str(&response_text).context("failed to deserialize response")
    }

    fn finalize_payload(&self, payload: Value) -> Value {
        let mut payload = match payload {
            Value::Object(map) => map,
            _ => serde_json::Map::new(),
        };

        payload.insert("e_r".to_string(), Value::Bool(true));
        payload.insert("header".to_string(), self.header_value());

        Value::Object(payload)
    }

    fn header_value(&self) -> Value {
        json!({
            "clientSign": self.client_sign,
            "os": self.os,
            "appver": self.app_version,
            "deviceId": self.device_id,
            "requestId": 0,
            "osver": self.os_version,
        })
    }
}

impl Default for NeteaseApiClient {
    fn default() -> Self {
        Self::new().expect("failed to create default netease api client")
    }
}

fn to_eapi_path(api_path: &str) -> String {
    if let Some(rest) = api_path.strip_prefix("/api/") {
        format!("/eapi/{rest}")
    } else {
        api_path.to_string()
    }
}

fn normalize_mac(mac: &str) -> String {
    let sanitized = mac.replace(':', "").replace('-', "").to_uppercase();
    if sanitized.is_empty() {
        "000000000000".to_string()
    } else {
        sanitized
    }
}

fn build_device_id(mac: &str) -> String {
    let digest = md5::compute(mac.as_bytes());
    format!("{}{:x}", mac, digest).to_uppercase()
}

fn build_client_sign(mac: &str) -> String {
    let digest = md5::compute(mac.as_bytes());
    format!("{}@@@RUST_NETEASE_CLIENT@@@@@@{:x}", mac, digest)
}

fn detect_os_version() -> String {
    format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH)
}

/// 将歌曲 id 列表编码为接口要求的字符串数组文本。
pub(crate) fn stringify_id_list(ids: &[i64]) -> String {
    Value::Array(ids.iter().map(|id| Value::String(id.to_string())).collect()).to_string()
}

#[cfg(test)]
mod tests {
    use super::{NeteaseApiClient, stringify_id_list, to_eapi_path};

    #[test]
    fn converts_api_path_to_eapi_path() {
        assert_eq!(to_eapi_path("/api/v2/banner/get"), "/eapi/v2/banner/get");
    }

    #[test]
    fn formats_id_list_for_eapi_payload() {
        assert_eq!(stringify_id_list(&[1, 2, 3]), r#"["1","2","3"]"#);
    }

    #[test]
    fn builds_default_client() {
        let client = NeteaseApiClient::new().unwrap();
        let payload = client.finalize_payload(serde_json::json!({"keyword": "test"}));
        assert!(payload.get("header").is_some());
        assert_eq!(payload.get("e_r").and_then(|v| v.as_bool()), Some(true));
    }
}
