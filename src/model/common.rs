use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 歌手摘要信息，供歌曲、专辑、搜索等多个接口复用。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtistSummary {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub alias: Vec<String>,
    #[serde(default)]
    pub alia: Vec<String>,
    #[serde(default)]
    pub tns: Vec<String>,
    #[serde(rename = "picId", default)]
    pub pic_id: Option<i64>,
    #[serde(rename = "img1v1Id", default)]
    pub img1v1_id: Option<i64>,
    #[serde(rename = "briefDesc", default)]
    pub brief_desc: Option<String>,
    #[serde(rename = "picUrl", default)]
    pub pic_url: Option<String>,
    #[serde(rename = "img1v1Url", default)]
    pub img1v1_url: Option<String>,
    #[serde(rename = "albumSize", default)]
    pub album_size: Option<i32>,
    #[serde(default)]
    pub trans: Option<String>,
    #[serde(rename = "musicSize", default)]
    pub music_size: Option<i32>,
    #[serde(rename = "topicPerson", default)]
    pub topic_person: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 专辑摘要信息，覆盖歌曲详情、专辑搜索等常见场景。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlbumSummary {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub size: Option<i32>,
    #[serde(rename = "picId", default)]
    pub pic_id: Option<i64>,
    #[serde(rename = "blurPicUrl", default)]
    pub blur_pic_url: Option<String>,
    #[serde(rename = "companyId", default)]
    pub company_id: Option<i64>,
    #[serde(default)]
    pub pic: Option<i64>,
    #[serde(rename = "picUrl", default)]
    pub pic_url: Option<String>,
    #[serde(rename = "publishTime", default)]
    pub publish_time: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Option<String>,
    #[serde(default)]
    pub company: Option<String>,
    #[serde(rename = "briefDesc", default)]
    pub brief_desc: Option<String>,
    #[serde(default)]
    pub artist: Option<ArtistSummary>,
    #[serde(default)]
    pub songs: Option<Vec<Value>>,
    #[serde(default)]
    pub alias: Vec<String>,
    #[serde(default)]
    pub artists: Vec<ArtistSummary>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 音质信息，描述某一档位的码率、大小与采样率。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioQuality {
    pub br: i64,
    pub fid: i64,
    pub size: i64,
    pub vd: f64,
    pub sr: i32,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 免费试听权限信息，常见于歌曲权限和播放地址返回值。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FreeTrialPrivilege {
    #[serde(rename = "resConsumable", default)]
    pub res_consumable: Option<bool>,
    #[serde(rename = "userConsumable", default)]
    pub user_consumable: Option<bool>,
    #[serde(rename = "listenType", default)]
    pub listen_type: Option<i32>,
    #[serde(rename = "cannotListenReason", default)]
    pub cannot_listen_reason: Option<i32>,
    #[serde(rename = "playReason", default)]
    pub play_reason: Option<String>,
    #[serde(rename = "freeLimitTagType", default)]
    pub free_limit_tag_type: Option<i32>,
    #[serde(rename = "type", default)]
    pub privilege_type: Option<i32>,
    #[serde(rename = "remainTime", default)]
    pub remain_time: Option<i64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 付费信息条目，描述不同码率下的计费状态。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChargeInfo {
    pub rate: i32,
    #[serde(rename = "chargeUrl", default)]
    pub charge_url: Option<String>,
    #[serde(rename = "chargeMessage", default)]
    pub charge_message: Option<String>,
    #[serde(rename = "chargeType", default)]
    pub charge_type: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 歌曲权限信息，包含播放、下载、试听和音质等级等字段。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SongPrivilege {
    pub id: i64,
    #[serde(default)]
    pub fee: Option<i32>,
    #[serde(default)]
    pub payed: Option<i32>,
    #[serde(rename = "realPayed", default)]
    pub real_payed: Option<i32>,
    #[serde(default)]
    pub st: Option<i32>,
    #[serde(default)]
    pub pl: Option<i32>,
    #[serde(default)]
    pub dl: Option<i32>,
    #[serde(default)]
    pub sp: Option<i32>,
    #[serde(default)]
    pub cp: Option<i32>,
    #[serde(default)]
    pub subp: Option<i32>,
    #[serde(default)]
    pub cs: Option<bool>,
    #[serde(default)]
    pub maxbr: Option<i32>,
    #[serde(default)]
    pub fl: Option<i32>,
    #[serde(default)]
    pub pc: Option<Value>,
    #[serde(default)]
    pub toast: Option<bool>,
    #[serde(default)]
    pub flag: Option<i64>,
    #[serde(rename = "paidBigBang", default)]
    pub paid_big_bang: Option<bool>,
    #[serde(rename = "preSell", default)]
    pub pre_sell: Option<bool>,
    #[serde(rename = "playMaxbr", default)]
    pub play_maxbr: Option<i32>,
    #[serde(rename = "downloadMaxbr", default)]
    pub download_maxbr: Option<i32>,
    #[serde(rename = "maxBrLevel", default)]
    pub max_br_level: Option<String>,
    #[serde(rename = "playMaxBrLevel", default)]
    pub play_max_br_level: Option<String>,
    #[serde(rename = "downloadMaxBrLevel", default)]
    pub download_max_br_level: Option<String>,
    #[serde(rename = "plLevel", default)]
    pub pl_level: Option<String>,
    #[serde(rename = "dlLevel", default)]
    pub dl_level: Option<String>,
    #[serde(rename = "flLevel", default)]
    pub fl_level: Option<String>,
    #[serde(default)]
    pub rscl: Option<Value>,
    #[serde(rename = "freeTrialPrivilege", default)]
    pub free_trial_privilege: Option<FreeTrialPrivilege>,
    #[serde(rename = "rightSource", default)]
    pub right_source: Option<i32>,
    #[serde(rename = "chargeInfoList", default)]
    pub charge_info_list: Vec<ChargeInfo>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 歌曲摘要信息，尽量兼容搜索结果和资源型接口中的歌曲结构。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SongSummary {
    pub name: String,
    pub id: i64,
    #[serde(default)]
    pub pst: Option<i32>,
    #[serde(default)]
    pub t: Option<i32>,
    #[serde(rename = "ar", default)]
    pub artists: Vec<ArtistSummary>,
    #[serde(default)]
    pub alia: Vec<String>,
    #[serde(default)]
    pub pop: Option<f64>,
    #[serde(default)]
    pub st: Option<i32>,
    #[serde(default)]
    pub rt: Option<String>,
    #[serde(default)]
    pub fee: Option<i32>,
    #[serde(default)]
    pub v: Option<i32>,
    #[serde(default)]
    pub crbt: Option<String>,
    #[serde(default)]
    pub cf: Option<String>,
    #[serde(rename = "al", default)]
    pub album: Option<AlbumSummary>,
    #[serde(rename = "dt", default)]
    pub duration: Option<i64>,
    #[serde(default)]
    pub h: Option<AudioQuality>,
    #[serde(default)]
    pub m: Option<AudioQuality>,
    #[serde(default)]
    pub l: Option<AudioQuality>,
    #[serde(default)]
    pub sq: Option<AudioQuality>,
    #[serde(default)]
    pub hr: Option<AudioQuality>,
    #[serde(default)]
    pub a: Option<Value>,
    #[serde(default)]
    pub cd: Option<String>,
    #[serde(default)]
    pub no: Option<i32>,
    #[serde(rename = "rtUrl", default)]
    pub rt_url: Option<String>,
    #[serde(rename = "ftype", default)]
    pub ftype: Option<i32>,
    #[serde(rename = "rtUrls", default)]
    pub rt_urls: Vec<String>,
    #[serde(rename = "djId", default)]
    pub dj_id: Option<i64>,
    #[serde(default)]
    pub copyright: Option<i32>,
    #[serde(rename = "s_id", default)]
    pub s_id: Option<i64>,
    #[serde(default)]
    pub mark: Option<i64>,
    #[serde(rename = "originCoverType", default)]
    pub origin_cover_type: Option<i32>,
    #[serde(rename = "originSongSimpleData", default)]
    pub origin_song_simple_data: Option<Value>,
    #[serde(rename = "tagPicList", default)]
    pub tag_pic_list: Option<Value>,
    #[serde(rename = "resourceState", default)]
    pub resource_state: Option<bool>,
    #[serde(default)]
    pub version: Option<i32>,
    #[serde(rename = "songJumpInfo", default)]
    pub song_jump_info: Option<Value>,
    #[serde(rename = "entertainmentTags", default)]
    pub entertainment_tags: Option<Value>,
    #[serde(rename = "awardTags", default)]
    pub award_tags: Option<Value>,
    #[serde(rename = "displayTags", default)]
    pub display_tags: Option<Value>,
    #[serde(rename = "markTags", default)]
    pub mark_tags: Vec<Value>,
    #[serde(default)]
    pub single: Option<i32>,
    #[serde(rename = "noCopyrightRcmd", default)]
    pub no_copyright_rcmd: Option<Value>,
    #[serde(default)]
    pub mst: Option<i32>,
    #[serde(default)]
    pub cp: Option<i32>,
    #[serde(default)]
    pub mv: Option<i64>,
    #[serde(default)]
    pub rtype: Option<i32>,
    #[serde(default)]
    pub rurl: Option<String>,
    #[serde(rename = "publishTime", default)]
    pub publish_time: Option<i64>,
    #[serde(default)]
    pub privilege: Option<SongPrivilege>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 用户资料摘要信息，覆盖普通用户、播客 DJ、创建者等场景。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile {
    #[serde(rename = "defaultAvatar", default)]
    pub default_avatar: Option<bool>,
    #[serde(default)]
    pub province: Option<i32>,
    #[serde(rename = "authStatus", default)]
    pub auth_status: Option<i32>,
    #[serde(default)]
    pub followed: Option<bool>,
    #[serde(rename = "avatarUrl", default)]
    pub avatar_url: Option<String>,
    #[serde(rename = "accountStatus", default)]
    pub account_status: Option<i32>,
    #[serde(default)]
    pub gender: Option<i32>,
    #[serde(default)]
    pub city: Option<i32>,
    #[serde(default)]
    pub birthday: Option<i64>,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "userType", default)]
    pub user_type: Option<i32>,
    pub nickname: String,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "detailDescription", default)]
    pub detail_description: Option<String>,
    #[serde(rename = "avatarImgId", default)]
    pub avatar_img_id: Option<i64>,
    #[serde(rename = "backgroundImgId", default)]
    pub background_img_id: Option<i64>,
    #[serde(rename = "backgroundUrl", default)]
    pub background_url: Option<String>,
    #[serde(default)]
    pub authority: Option<i32>,
    #[serde(default)]
    pub mutual: Option<bool>,
    #[serde(rename = "expertTags", default)]
    pub expert_tags: Option<Vec<String>>,
    #[serde(default)]
    pub experts: Option<Value>,
    #[serde(rename = "djStatus", default)]
    pub dj_status: Option<i32>,
    #[serde(rename = "vipType", default)]
    pub vip_type: Option<i32>,
    #[serde(rename = "remarkName", default)]
    pub remark_name: Option<String>,
    #[serde(rename = "authenticationTypes", default)]
    pub authentication_types: Option<i32>,
    #[serde(rename = "avatarDetail", default)]
    pub avatar_detail: Option<Value>,
    #[serde(default)]
    pub anchor: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 歌单摘要信息，主要用于搜索结果和推荐列表。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaylistSummary {
    pub id: i64,
    pub name: String,
    #[serde(rename = "coverImgUrl", default)]
    pub cover_img_url: Option<String>,
    #[serde(default)]
    pub creator: Option<UserProfile>,
    #[serde(default)]
    pub subscribed: Option<bool>,
    #[serde(rename = "trackCount", default)]
    pub track_count: Option<i32>,
    #[serde(rename = "userId", default)]
    pub user_id: Option<i64>,
    #[serde(rename = "playCount", default)]
    pub play_count: Option<i64>,
    #[serde(rename = "bookCount", default)]
    pub book_count: Option<i64>,
    #[serde(rename = "specialType", default)]
    pub special_type: Option<i32>,
    #[serde(rename = "officialTags", default)]
    pub official_tags: Option<Vec<String>>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(rename = "actionType", default)]
    pub action_type: Option<String>,
    #[serde(rename = "recommendText", default)]
    pub recommend_text: Option<String>,
    #[serde(default)]
    pub score: Option<Value>,
    #[serde(rename = "officialPlaylistTitle", default)]
    pub official_playlist_title: Option<String>,
    #[serde(rename = "playlistType", default)]
    pub playlist_type: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "highQuality", default)]
    pub high_quality: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 通用标题展示模型，对应搜索结果中的标题节点。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiTitle {
    pub title: String,
    #[serde(rename = "leftIconUrl", default)]
    pub left_icon_url: Option<String>,
    #[serde(rename = "showType", default)]
    pub show_type: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 通用图片展示模型，对应搜索结果中的封面节点。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiImage {
    #[serde(default)]
    pub action: Option<String>,
    #[serde(rename = "actionUrl", default)]
    pub action_url: Option<String>,
    #[serde(rename = "actionType", default)]
    pub action_type: Option<String>,
    #[serde(rename = "imageUrl", default)]
    pub image_url: Option<String>,
    #[serde(rename = "imageType", default)]
    pub image_type: Option<String>,
    #[serde(default)]
    pub width: Option<i32>,
    #[serde(default)]
    pub height: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 通用 UI 展示模型，承载搜索结果卡片中的文案与图片信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiElement {
    #[serde(rename = "mainTitle", default)]
    pub main_title: Option<UiTitle>,
    #[serde(rename = "subTitle", default)]
    pub sub_title: Option<Value>,
    #[serde(default)]
    pub button: Option<Value>,
    #[serde(default)]
    pub more: Option<Value>,
    #[serde(default)]
    pub image: Option<UiImage>,
    #[serde(default)]
    pub tag: Option<Value>,
    #[serde(rename = "imageTag", default)]
    pub image_tag: Option<Value>,
    #[serde(default)]
    pub desc: Option<Value>,
    #[serde(rename = "rightButton", default)]
    pub right_button: Option<Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 通用搜索资源包装模型，适用于多种资源类型的统一解析。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchResource<T> {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "resourceType", default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(rename = "resourceName", default)]
    pub resource_name: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(rename = "actionType", default)]
    pub action_type: Option<String>,
    #[serde(rename = "uiElement", default)]
    pub ui_element: Option<UiElement>,
    #[serde(rename = "baseInfo")]
    pub base_info: T,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
