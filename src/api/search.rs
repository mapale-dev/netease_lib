use anyhow::Result;
use serde_json::json;

use crate::{
    api::client::NeteaseApiClient,
    model::search::{
        AlbumSearchResponse, ArtistSearchResponse, LyricResourceSearchResponse,
        MlogSearchResponse, PlaylistSearchResponse, SongListSearchResponse, UserSearchResponse,
        VoiceListSearchResponse, VoiceSearchResponse,
    },
};

/// 通用搜索关键字分页参数。
#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub keyword: String,
    pub limit: u32,
    pub offset: u32,
}

impl SearchQuery {
    /// 使用关键字创建默认搜索参数。
    pub fn new(keyword: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            limit: 10,
            offset: 0,
        }
    }

    /// 设置分页大小。
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// 设置分页偏移。
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }
}

/// Mlog 搜索参数。
#[derive(Debug, Clone)]
pub struct MlogQuery {
    pub keyword: String,
    pub limit: u32,
    pub offset: u32,
    pub scene: String,
    pub channel: String,
    pub os: String,
    pub tag: String,
}

impl MlogQuery {
    /// 使用关键字创建默认 Mlog 搜索参数。
    pub fn new(keyword: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            limit: 20,
            offset: 0,
            scene: "normal".to_string(),
            channel: "suggest".to_string(),
            os: "pc".to_string(),
            tag: "MV".to_string(),
        }
    }

    /// 设置标签。
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = tag.into();
        self
    }
}

/// 搜索专辑。
pub async fn album(client: &NeteaseApiClient, query: &SearchQuery) -> Result<AlbumSearchResponse> {
    client
        .post_eapi(
            "/api/v1/search/album/get",
            json!({
                "s": query.keyword,
                "limit": query.limit.to_string(),
                "offset": query.offset.to_string(),
                "queryCorrect": "true",
            }),
        )
        .await
}

/// 搜索歌手。
pub async fn artist(client: &NeteaseApiClient, query: &SearchQuery) -> Result<ArtistSearchResponse> {
    client
        .post_eapi(
            "/api/v1/search/artist/get",
            json!({
                "s": query.keyword,
                "limit": query.limit.to_string(),
                "offset": query.offset.to_string(),
                "queryCorrect": "true",
            }),
        )
        .await
}

/// 搜索歌单。
pub async fn playlist(client: &NeteaseApiClient, query: &SearchQuery) -> Result<PlaylistSearchResponse> {
    client
        .post_eapi(
            "/api/v1/search/playlist/get",
            json!({
                "s": query.keyword,
                "limit": query.limit.to_string(),
                "offset": query.offset.to_string(),
                "queryCorrect": "true",
                "checkToken": "",
            }),
        )
        .await
}

/// 搜索用户。
pub async fn user(client: &NeteaseApiClient, query: &SearchQuery) -> Result<UserSearchResponse> {
    client
        .post_eapi(
            "/api/v1/search/user/get",
            json!({
                "s": query.keyword,
                "limit": query.limit.to_string(),
                "offset": query.offset.to_string(),
                "queryCorrect": "true",
            }),
        )
        .await
}

/// 搜索单曲列表页。
pub async fn song_list_page(
    client: &NeteaseApiClient,
    query: &SearchQuery,
) -> Result<SongListSearchResponse> {
    client
        .post_eapi(
            "/api/search/song/list/page",
            json!({
                "keyword": query.keyword,
                "scene": "normal",
                "limit": query.limit.to_string(),
                "offset": query.offset.to_string(),
                "needCorrect": "true",
                "channel": "suggest",
                "checkToken": "",
            }),
        )
        .await
}

/// 搜索歌词资源。
pub async fn resource_lyric(
    client: &NeteaseApiClient,
    query: &SearchQuery,
) -> Result<LyricResourceSearchResponse> {
    client
        .post_eapi(
            "/api/search/resource/lyric",
            json!({
                "keyword": query.keyword,
                "scene": "normal",
                "limit": query.limit.to_string(),
                "offset": query.offset.to_string(),
            }),
        )
        .await
}

/// 搜索声音节目。
pub async fn voice(client: &NeteaseApiClient, query: &SearchQuery) -> Result<VoiceSearchResponse> {
    client
        .post_eapi(
            "/api/search/voice/get",
            json!({
                "keyword": query.keyword,
                "scene": "normal",
                "limit": query.limit.to_string(),
                "offset": query.offset.to_string(),
            }),
        )
        .await
}

/// 搜索播客列表。
pub async fn voicelist(
    client: &NeteaseApiClient,
    query: &SearchQuery,
) -> Result<VoiceListSearchResponse> {
    client
        .post_eapi(
            "/api/search/voicelist/get",
            json!({
                "keyword": query.keyword,
                "scene": "normal",
                "limit": query.limit.to_string(),
                "offset": query.offset.to_string(),
            }),
        )
        .await
}

/// 搜索 Mlog / MV 资源。
pub async fn mlog(
    client: &NeteaseApiClient,
    query: &SearchQuery,
    tag: Option<&str>,
) -> Result<MlogSearchResponse> {
    let request = MlogQuery::new(query.keyword.clone())
        .with_tag(tag.unwrap_or("MV"))
        .with_limit(query.limit)
        .with_offset(query.offset);

    fetch_mlog(client, &request).await
}

impl MlogQuery {
    /// 设置分页大小。
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// 设置分页偏移。
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }
}

/// 按结构化参数搜索 Mlog / MV 资源。
pub async fn fetch_mlog(client: &NeteaseApiClient, query: &MlogQuery) -> Result<MlogSearchResponse> {
    client
        .post_eapi(
            "/api/search/mlog/get",
            json!({
                "keyword": query.keyword,
                "scene": query.scene,
                "limit": query.limit.to_string(),
                "offset": query.offset.to_string(),
                "channel": query.channel,
                "os": query.os,
                "tag": query.tag,
            }),
        )
        .await
}
