use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::common::{
    AlbumSummary, ArtistSummary, PlaylistSummary, SearchResource, SongSummary, UiElement,
    UserProfile,
};

/// 专辑搜索结果中的单条专辑数据。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlbumSearchItem {
    #[serde(flatten)]
    pub album: AlbumSummary,
    #[serde(default)]
    pub paid: Option<bool>,
    #[serde(rename = "onSale", default)]
    pub on_sale: Option<bool>,
    #[serde(default)]
    pub alg: Option<String>,
    #[serde(rename = "containedSong", default)]
    pub contained_song: Option<String>,
}

/// 专辑搜索结果主体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlbumSearchResult {
    #[serde(rename = "hlWords", default)]
    pub hl_words: Vec<String>,
    #[serde(default)]
    pub albums: Vec<AlbumSearchItem>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 专辑搜索接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlbumSearchResponse {
    pub result: AlbumSearchResult,
    pub code: i32,
}

/// 歌手搜索结果中的单条歌手数据。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtistSearchItem {
    #[serde(flatten)]
    pub artist: ArtistSummary,
    #[serde(rename = "fansGroup", default)]
    pub fans_group: Option<Value>,
    #[serde(rename = "recommendText", default)]
    pub recommend_text: Option<String>,
    #[serde(rename = "appendRecText", default)]
    pub append_rec_text: Option<String>,
    #[serde(rename = "fansSize", default)]
    pub fans_size: Option<i64>,
    #[serde(rename = "accountId", default)]
    pub account_id: Option<i64>,
    #[serde(rename = "identityIconUrl", default)]
    pub identity_icon_url: Option<String>,
    #[serde(rename = "mvSize", default)]
    pub mv_size: Option<i32>,
    #[serde(default)]
    pub followed: Option<bool>,
    #[serde(default)]
    pub alg: Option<String>,
}

/// 歌手搜索结果主体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtistSearchResult {
    #[serde(rename = "hasMore", default)]
    pub has_more: Option<bool>,
    #[serde(rename = "artistCount", default)]
    pub artist_count: Option<i32>,
    #[serde(rename = "hlWords", default)]
    pub hl_words: Vec<String>,
    #[serde(default)]
    pub artists: Vec<ArtistSearchItem>,
    #[serde(rename = "searchQcReminder", default)]
    pub search_qc_reminder: Option<Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 歌手搜索接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtistSearchResponse {
    pub result: ArtistSearchResult,
    pub code: i32,
}

/// 歌单搜索结果主体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaylistSearchResult {
    #[serde(default)]
    pub playlists: Vec<PlaylistSummary>,
    #[serde(rename = "playlistCount", default)]
    pub playlist_count: Option<i32>,
    #[serde(rename = "hasMore", default)]
    pub has_more: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 歌单搜索接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaylistSearchResponse {
    pub result: PlaylistSearchResult,
    pub code: i32,
}

/// 用户搜索结果主体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserSearchResult {
    #[serde(rename = "hasMore", default)]
    pub has_more: Option<bool>,
    #[serde(rename = "userprofileCount", default)]
    pub userprofile_count: Option<i32>,
    #[serde(rename = "hlWords", default)]
    pub hl_words: Vec<String>,
    #[serde(rename = "userprofiles", default)]
    pub user_profiles: Vec<UserProfile>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 用户搜索接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserSearchResponse {
    pub result: UserSearchResult,
    pub code: i32,
}

/// 单曲搜索资源中的歌曲主体信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchSongBaseInfo {
    #[serde(rename = "simpleSongData")]
    pub simple_song_data: SongSummary,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 单曲搜索结果中的单条资源数据。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchSongResource {
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "baseInfo")]
    pub base_info: SearchSongBaseInfo,
    #[serde(rename = "uiElement", default)]
    pub ui_element: Option<UiElement>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 通用搜索数据容器，内部包含资源列表与扩展字段。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchData<T> {
    #[serde(default)]
    pub resources: Vec<T>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 单曲列表搜索接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SongListSearchResponse {
    pub code: i32,
    pub data: SearchData<SearchSongResource>,
}

/// 歌词资源搜索中的单条资源数据。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LyricResourceItem {
    pub song: SongSummary,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 歌词资源搜索接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LyricResourceSearchResponse {
    pub code: i32,
    #[serde(default)]
    pub message: Option<String>,
    pub data: SearchData<LyricResourceItem>,
}

/// 声音搜索结果中的基础资源信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VoiceBaseInfo {
    #[serde(rename = "mainSong")]
    pub main_song: Value,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 声音搜索结果资源类型别名。
pub type VoiceSearchResource = SearchResource<VoiceBaseInfo>;

/// 声音搜索接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VoiceSearchResponse {
    pub code: i32,
    pub data: SearchData<VoiceSearchResource>,
}

/// 播客搜索结果中的基础资源信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VoiceListBaseInfo {
    pub id: i64,
    pub dj: UserProfile,
    pub name: String,
    #[serde(rename = "picUrl", default)]
    pub pic_url: Option<String>,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(rename = "subCount", default)]
    pub sub_count: Option<i64>,
    #[serde(rename = "programCount", default)]
    pub program_count: Option<i32>,
    #[serde(rename = "createTime", default)]
    pub create_time: Option<i64>,
    #[serde(rename = "categoryId", default)]
    pub category_id: Option<i64>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(rename = "secondCategoryId", default)]
    pub second_category_id: Option<i64>,
    #[serde(rename = "secondCategory", default)]
    pub second_category: Option<String>,
    #[serde(rename = "radioFeeType", default)]
    pub radio_fee_type: Option<i32>,
    #[serde(rename = "feeScope", default)]
    pub fee_scope: Option<i32>,
    #[serde(default)]
    pub buyed: Option<bool>,
    #[serde(default)]
    pub videos: Option<Value>,
    #[serde(default)]
    pub finished: Option<bool>,
    #[serde(rename = "underShelf", default)]
    pub under_shelf: Option<bool>,
    #[serde(rename = "purchaseCount", default)]
    pub purchase_count: Option<i64>,
    #[serde(default)]
    pub price: Option<i32>,
    #[serde(rename = "originalPrice", default)]
    pub original_price: Option<i32>,
    #[serde(rename = "discountPrice", default)]
    pub discount_price: Option<Value>,
    #[serde(rename = "lastProgramCreateTime", default)]
    pub last_program_create_time: Option<i64>,
    #[serde(rename = "lastProgramName", default)]
    pub last_program_name: Option<String>,
    #[serde(rename = "lastProgramId", default)]
    pub last_program_id: Option<i64>,
    #[serde(rename = "picId", default)]
    pub pic_id: Option<i64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 播客搜索结果资源类型别名。
pub type VoiceListSearchResource = SearchResource<VoiceListBaseInfo>;

/// 播客搜索接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VoiceListSearchResponse {
    pub code: i32,
    pub data: SearchData<VoiceListSearchResource>,
}

/// Mlog 视频清晰度条目。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MlogVideo {
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub width: Option<i32>,
    #[serde(default)]
    pub height: Option<i32>,
    #[serde(default)]
    pub container: Option<String>,
    #[serde(default)]
    pub md5: Option<String>,
    #[serde(default)]
    pub check: Option<bool>,
    #[serde(rename = "tagSign", default)]
    pub tag_sign: Option<Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Mlog 主体数据，包含封面、文案、视频列表等字段。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MlogBaseData {
    pub id: String,
    #[serde(rename = "userId", default)]
    pub user_id: Option<i64>,
    #[serde(rename = "type", default)]
    pub data_type: Option<i32>,
    #[serde(rename = "originalTitle", default)]
    pub original_title: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(rename = "interveneText", default)]
    pub intervene_text: Option<String>,
    #[serde(rename = "pubTime", default)]
    pub pub_time: Option<i64>,
    #[serde(rename = "coverUrl", default)]
    pub cover_url: Option<String>,
    #[serde(rename = "coverDetail", default)]
    pub cover_detail: Option<Value>,
    #[serde(rename = "coverHeight", default)]
    pub cover_height: Option<i32>,
    #[serde(rename = "greatCover", default)]
    pub great_cover: Option<bool>,
    #[serde(rename = "coverWidth", default)]
    pub cover_width: Option<i32>,
    #[serde(rename = "coverColor", default)]
    pub cover_color: Option<i32>,
    #[serde(rename = "coverPicKey", default)]
    pub cover_pic_key: Option<String>,
    #[serde(rename = "coverDynamicUrl", default)]
    pub cover_dynamic_url: Option<String>,
    #[serde(default)]
    pub audio: Option<Value>,
    #[serde(rename = "threadId", default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub duration: Option<i64>,
    #[serde(default)]
    pub video: Option<Value>,
    #[serde(default)]
    pub videos: Option<Vec<MlogVideo>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Mlog 资源包装体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MlogResourceBody {
    #[serde(rename = "mlogBaseData")]
    pub mlog_base_data: MlogBaseData,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Mlog 搜索结果中的基础资源信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MlogBaseInfo {
    pub id: String,
    #[serde(rename = "type", default)]
    pub info_type: Option<i32>,
    #[serde(rename = "mlogBaseDataType", default)]
    pub mlog_base_data_type: Option<i32>,
    #[serde(default)]
    pub position: Option<i32>,
    #[serde(default)]
    pub resource: Option<MlogResourceBody>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Mlog 搜索结果资源类型别名。
pub type MlogSearchResource = SearchResource<MlogBaseInfo>;

/// Mlog 搜索接口响应体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MlogSearchResponse {
    pub code: i32,
    pub data: SearchData<MlogSearchResource>,
}
