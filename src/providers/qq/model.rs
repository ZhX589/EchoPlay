use serde::Deserialize;

// ---------------------------------------------------------------------------
// Search response
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub code: Option<i64>,
    pub search: Option<SearchInner>,
}

#[derive(Debug, Deserialize)]
pub struct SearchInner {
    pub data: Option<SearchData>,
}

#[derive(Debug, Deserialize)]
pub struct SearchData {
    pub body: Option<SearchBody>,
}

#[derive(Debug, Deserialize)]
pub struct SearchBody {
    pub song: Option<SearchSongList>,
    pub singer: Option<SearchSingerList>,
    pub album: Option<SearchAlbumList>,
    pub songlist: Option<SearchPlaylistList>,
}

#[derive(Debug, Deserialize)]
pub struct SearchSongList {
    pub list: Option<Vec<SongItem>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchSingerList {
    pub list: Option<Vec<SingerItem>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchAlbumList {
    pub list: Option<Vec<AlbumItem>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchPlaylistList {
    pub list: Option<Vec<PlaylistItem>>,
}

// ---------------------------------------------------------------------------
// Song item (from search or detail)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct SongItem {
    pub id: Option<i64>,
    pub mid: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub singer: Option<Vec<SingerRef>>,
    pub album: Option<AlbumRef>,
    pub mv: Option<MvRef>,
    /// Duration in seconds.
    pub interval: Option<i64>,
    pub file: Option<FileInfo>,
    pub pay: Option<PayInfo>,
    pub time_public: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SingerRef {
    pub id: Option<i64>,
    pub mid: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumRef {
    pub id: Option<i64>,
    pub mid: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MvRef {
    pub id: Option<i64>,
    pub vid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileInfo {
    pub media_mid: Option<String>,
    pub size_128mp3: Option<i64>,
    pub size_320mp3: Option<i64>,
    pub size_flac: Option<i64>,
    pub size_ape: Option<i64>,
    pub size_192aac: Option<i64>,
    pub size_hires: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PayInfo {
    pub pay_play: Option<i64>,
}

// ---------------------------------------------------------------------------
// Album item (from search)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumItem {
    #[serde(rename = "albumID")]
    pub album_id: Option<i64>,
    #[serde(rename = "albumMID")]
    pub album_mid: Option<String>,
    #[serde(rename = "albumName")]
    pub album_name: Option<String>,
    #[serde(rename = "albumPic")]
    pub album_pic: Option<String>,
    #[serde(rename = "publicTime")]
    pub public_time: Option<String>,
    pub song_count: Option<usize>,
    pub singer_list: Option<Vec<SingerRef>>,
}

// ---------------------------------------------------------------------------
// Singer/Artist item (from search)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct SingerItem {
    #[serde(rename = "singerID")]
    pub singer_id: Option<i64>,
    #[serde(rename = "singerMID")]
    pub singer_mid: Option<String>,
    #[serde(rename = "singerName")]
    pub singer_name: Option<String>,
    #[serde(rename = "singerPic")]
    pub singer_pic: Option<String>,
}

// ---------------------------------------------------------------------------
// Playlist item (from search)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct PlaylistItem {
    pub dissid: Option<String>,
    pub dissname: Option<String>,
    pub imgurl: Option<String>,
    pub introduction: Option<String>,
    pub listennum: Option<u64>,
    pub song_count: Option<usize>,
    pub creator: Option<CreatorRef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatorRef {
    pub name: Option<String>,
}

// ---------------------------------------------------------------------------
// Song detail response (RPC)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct SongDetailResponse {
    pub code: Option<i64>,
    pub detail: Option<SongDetailInner>,
}

#[derive(Debug, Deserialize)]
pub struct SongDetailInner {
    pub data: Option<SongDetailData>,
}

#[derive(Debug, Deserialize)]
pub struct SongDetailData {
    pub track_info: Option<SongItem>,
}

// ---------------------------------------------------------------------------
// Media URL response (RPC vkey)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct VkeyResponse {
    pub code: Option<i64>,
    pub req_0: Option<VkeyReq>,
}

#[derive(Debug, Deserialize)]
pub struct VkeyReq {
    pub data: Option<VkeyData>,
}

#[derive(Debug, Deserialize)]
pub struct VkeyData {
    pub midurlinfo: Option<Vec<MidUrlInfo>>,
    pub sip: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct MidUrlInfo {
    pub purl: Option<String>,
}

// ---------------------------------------------------------------------------
// Lyric response (legacy REST)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct LyricResponse {
    pub code: Option<i64>,
    pub lyric: Option<String>,
    pub trans: Option<String>,
}

// ---------------------------------------------------------------------------
// Album detail response (legacy REST)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct AlbumDetailResponse {
    pub code: Option<i64>,
    pub data: Option<AlbumDetailData>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumDetailData {
    pub songlist: Option<Vec<SongItem>>,
    pub color: Option<i64>,
    pub genre: Option<String>,
}

// ---------------------------------------------------------------------------
// Helper: convert raw JSON into our response types
// ---------------------------------------------------------------------------

/// Parse the RPC gateway response. The gateway wraps each request key in the
/// response, so we need to extract the right key.
pub fn parse_rpc_response<T: serde::de::DeserializeOwned>(
    value: &serde_json::Value,
    key: &str,
) -> Result<T, serde_json::Error> {
    serde_json::from_value(value[key].clone())
}
