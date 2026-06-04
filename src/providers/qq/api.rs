use std::collections::HashMap;

use base64::Engine;
use md5::{Digest, Md5};
use rand::Rng;
use reqwest::Client;
use serde_json::json;

use crate::error::EchoError;
use crate::media::{AudioQuality, Media};
use crate::model::*;

use super::model::*;

const RPC_URL: &str = "https://u.y.qq.com/cgi-bin/musicu.fcg";
const LYRIC_URL: &str = "https://c.y.qq.com/lyric/fcgi-bin/fcg_query_lyric_new.fcg";

/// Quality descriptor: (EchoPlay quality, filename prefix, extension).
const QUALITY_MAP: &[(AudioQuality, &str, &str)] = &[
    (AudioQuality::Shq, "F000", "flac"),
    (AudioQuality::Hq, "M800", "mp3"),
    (AudioQuality::Sq, "C600", "m4a"),
    (AudioQuality::Lq, "M500", "mp3"),
];

pub struct QQMusicApi {
    client: Client,
    uin: String,
    guid: String,
}

impl QQMusicApi {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()
            .expect("failed to build HTTP client");

        Self {
            client,
            uin: "0".into(),
            guid: random_guid(),
        }
    }

    pub fn with_cookies(cookies: HashMap<String, String>) -> Self {
        let mut api = Self::new();
        if let Some(uin) = cookies.get("uin") {
            api.uin = uin.clone();
        }
        api
    }

    // -----------------------------------------------------------------------
    // Search
    // -----------------------------------------------------------------------

    pub async fn search(
        &self,
        keyword: &str,
        search_type: SearchType,
    ) -> Result<SearchResult, EchoError> {
        let qq_type = match search_type {
            SearchType::Song => 0,
            SearchType::Artist => 1,
            SearchType::Album => 2,
            SearchType::Playlist => 3,
            _ => 0,
        };

        let payload = json!({
            "search": {
                "method": "DoSearchForQQMusicDesktop",
                "module": "music.search.SearchCgiService",
                "param": {
                    "num_per_page": 20,
                    "page_num": 1,
                    "search_type": qq_type,
                    "query": keyword,
                }
            },
            "comm": self.comm(),
        });

        let resp: serde_json::Value = self.rpc_request(&payload).await?;
        let mut result = SearchResult::empty("qq", keyword, search_type);

        if let Some(search) = resp.get("search")
            && let Some(body) = search.pointer("/data/body")
        {
            // Songs
            if let Some(list) = body
                .pointer("/song/list")
                .and_then(|v| serde_json::from_value::<Vec<SongItem>>(v.clone()).ok())
            {
                result.songs = list.iter().map(Self::song_item_to_brief).collect();
            }
            // Artists
            if let Some(list) = body
                .pointer("/singer/list")
                .and_then(|v| serde_json::from_value::<Vec<SingerItem>>(v.clone()).ok())
            {
                result.artists = list.iter().map(Self::singer_item_to_brief).collect();
            }
            // Albums
            if let Some(list) = body
                .pointer("/album/list")
                .and_then(|v| serde_json::from_value::<Vec<AlbumItem>>(v.clone()).ok())
            {
                result.albums = list.iter().map(Self::album_item_to_brief).collect();
            }
            // Playlists
            if let Some(list) = body
                .pointer("/songlist/list")
                .and_then(|v| serde_json::from_value::<Vec<PlaylistItem>>(v.clone()).ok())
            {
                result.playlists = list.iter().map(Self::playlist_item_to_brief).collect();
            }
        }

        Ok(result)
    }

    // -----------------------------------------------------------------------
    // Song detail
    // -----------------------------------------------------------------------

    pub async fn get_song_detail(&self, mid: &str) -> Result<Song, EchoError> {
        // Search by songmid to get the song info
        let search_result = self.search(mid, SearchType::Song).await?;
        if let Some(brief) = search_result.songs.first() {
            return Ok(Song {
                identifier: brief.identifier.clone(),
                source: "qq".into(),
                title: brief.title.clone(),
                artists_name: brief.artists_name.clone(),
                album_name: brief.album_name.clone(),
                duration_ms: brief.duration_ms.clone(),
                album: None,
                artists: vec![],
                duration: brief.duration_ms.parse().unwrap_or(0),
                genre: String::new(),
                date: String::new(),
                track: String::new(),
                disc: String::new(),
                pic_url: String::new(),
            });
        }

        Err(EchoError::ModelNotFound {
            reason: crate::error::NotFoundReason::NotFound,
        })
    }

    // -----------------------------------------------------------------------
    // Quality detection
    // -----------------------------------------------------------------------

    pub fn detect_qualities(file: &FileInfo) -> Vec<AudioQuality> {
        let mut qualities = Vec::new();
        if file.size_flac.unwrap_or(0) > 0 || file.size_ape.unwrap_or(0) > 0 {
            qualities.push(AudioQuality::Shq);
        }
        if file.size_320mp3.unwrap_or(0) > 0 {
            qualities.push(AudioQuality::Hq);
        }
        if file.size_192aac.unwrap_or(0) > 0 {
            qualities.push(AudioQuality::Sq);
        }
        if file.size_128mp3.unwrap_or(0) > 0 {
            qualities.push(AudioQuality::Lq);
        }
        qualities
    }

    // -----------------------------------------------------------------------
    // Media URL
    // -----------------------------------------------------------------------

    pub async fn get_song_media_url(
        &self,
        songmid: &str,
        media_mid: &str,
        quality: AudioQuality,
    ) -> Result<Option<Media>, EchoError> {
        let (prefix, ext) = match QUALITY_MAP.iter().find(|(q, _, _)| *q == quality) {
            Some((_, p, e)) => (*p, *e),
            None => return Ok(None),
        };

        let filename = format!("{}{}{}.{}", prefix, media_mid, "", ext);

        let payload = json!({
            "req_0": {
                "module": "vkey.GetVkeyServer",
                "method": "CgiGetVkey",
                "param": {
                    "filename": [filename],
                    "guid": self.guid,
                    "songmid": [songmid],
                    "songtype": [0],
                    "uin": self.uin,
                    "loginflag": 1,
                    "platform": "20",
                }
            },
            "comm": self.comm(),
        });

        let resp: serde_json::Value = self.rpc_request(&payload).await?;

        if let Some(req_0) = resp.get("req_0")
            && let (Some(purl), Some(sip)) = (
                req_0
                    .pointer("/data/midurlinfo/0/purl")
                    .and_then(|v| v.as_str()),
                req_0.pointer("/data/sip/0").and_then(|v| v.as_str()),
            )
            && !purl.is_empty()
        {
            let url = format!("{}{}", sip, purl);
            return Ok(Some(Media::audio(url)));
        }

        Ok(None)
    }

    // -----------------------------------------------------------------------
    // Lyrics
    // -----------------------------------------------------------------------

    pub async fn get_lyric(&self, songmid: &str) -> Result<Option<Lyric>, EchoError> {
        let ts = chrono_now_ms();
        let url = format!(
            "{}?songmid={}&pcachetime={}&format=json",
            LYRIC_URL, songmid, ts
        );

        let resp = self
            .client
            .get(&url)
            .header("Referer", "http://y.qq.com/")
            .send()
            .await
            .map_err(|e| EchoError::ProviderIO {
                message: "lyric request failed".into(),
                source: Some(anyhow::anyhow!(e)),
            })?;

        let json: serde_json::Value = resp.json().await.map_err(|e| EchoError::ProviderIO {
            message: "lyric parse failed".into(),
            source: Some(anyhow::anyhow!(e)),
        })?;

        let lyric_b64 = json.get("lyric").and_then(|v| v.as_str()).unwrap_or("");
        let trans_b64 = json.get("trans").and_then(|v| v.as_str()).unwrap_or("");

        if lyric_b64.is_empty() {
            return Ok(None);
        }

        let engine = base64::engine::general_purpose::STANDARD;
        let content = engine
            .decode(lyric_b64)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .unwrap_or_default();

        let trans_content = engine
            .decode(trans_b64)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .unwrap_or_default();

        Ok(Some(Lyric {
            identifier: songmid.to_string(),
            source: "qq".into(),
            content,
            trans_content,
        }))
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    async fn rpc_request(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, EchoError> {
        let data_str = serde_json::to_string(payload).map_err(|e| EchoError::ProviderIO {
            message: "serialize payload".into(),
            source: Some(anyhow::anyhow!(e)),
        })?;

        let sign = compute_sign(&data_str);
        let ts = chrono_now_ms();

        let url = format!("{}?_={}&sign={}&data={}", RPC_URL, ts, sign, data_str);

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| EchoError::ProviderIO {
                message: "RPC request failed".into(),
                source: Some(anyhow::anyhow!(e)),
            })?;

        let json: serde_json::Value = resp.json().await.map_err(|e| EchoError::ProviderIO {
            message: "RPC response parse failed".into(),
            source: Some(anyhow::anyhow!(e)),
        })?;

        Ok(json)
    }

    fn comm(&self) -> serde_json::Value {
        json!({
            "loginUin": self.uin,
            "hostUin": 0,
            "g_tk": 5381,
            "inCharset": "utf8",
            "outCharset": "utf-8",
            "notice": 0,
            "platform": "yqq",
            "needNewCode": 0,
        })
    }

    fn song_item_to_brief(item: &SongItem) -> BriefSong {
        let artists_name = item
            .singer
            .as_ref()
            .map(|singers| {
                singers
                    .iter()
                    .filter_map(|s| s.name.clone())
                    .collect::<Vec<_>>()
                    .join(" / ")
            })
            .unwrap_or_default();

        let album_name = item
            .album
            .as_ref()
            .and_then(|a| a.name.clone())
            .unwrap_or_default();

        let mid = item.mid.clone().unwrap_or_default();

        BriefSong {
            identifier: mid.clone(),
            source: "qq".into(),
            title: item
                .name
                .clone()
                .unwrap_or_else(|| item.title.clone().unwrap_or_default()),
            artists_name,
            album_name,
            duration_ms: item
                .interval
                .map(|i| (i * 1000).to_string())
                .unwrap_or_default(),
        }
    }

    fn singer_item_to_brief(item: &SingerItem) -> BriefArtist {
        BriefArtist {
            identifier: item.singer_mid.clone().unwrap_or_default(),
            source: "qq".into(),
            name: item.singer_name.clone().unwrap_or_default(),
        }
    }

    fn album_item_to_brief(item: &AlbumItem) -> BriefAlbum {
        let artists_name = item
            .singer_list
            .as_ref()
            .map(|singers| {
                singers
                    .iter()
                    .filter_map(|s| s.name.clone())
                    .collect::<Vec<_>>()
                    .join(" / ")
            })
            .unwrap_or_default();

        BriefAlbum {
            identifier: item.album_mid.clone().unwrap_or_default(),
            source: "qq".into(),
            name: item.album_name.clone().unwrap_or_default(),
            artists_name,
        }
    }

    fn playlist_item_to_brief(item: &PlaylistItem) -> BriefPlaylist {
        BriefPlaylist {
            identifier: item.dissid.clone().unwrap_or_default(),
            source: "qq".into(),
            name: item.dissname.clone().unwrap_or_default(),
            creator_name: item
                .creator
                .as_ref()
                .and_then(|c| c.name.clone())
                .unwrap_or_default(),
        }
    }
}

impl Default for QQMusicApi {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Utility functions
// ---------------------------------------------------------------------------

fn random_guid() -> String {
    let mut rng = rand::thread_rng();
    let num: u64 = rng.gen_range(100_000_000..999_999_999);
    num.to_string()
}

fn chrono_now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Compute the `sign` parameter for the RPC gateway.
/// Algorithm: "zza" + 10-16 random chars + md5("CJBPACrRuNy7" + data_str)
fn compute_sign(data: &str) -> String {
    let mut rng = rand::thread_rng();
    let chars: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let prefix_len = rng.gen_range(10..=16);

    let mut sign = String::from("zza");
    for _ in 0..prefix_len {
        let idx = rng.gen_range(0..chars.len());
        sign.push(chars[idx] as char);
    }

    let mut hasher = Md5::new();
    hasher.update(format!("CJBPACrRuNy7{}", data));
    let hash = format!("{:x}", hasher.finalize());
    sign.push_str(&hash);
    sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sign_format() {
        let sign = compute_sign(r#"{"test":1}"#);
        assert!(sign.starts_with("zza"));
        // zza (3) + 10-16 random + 32 md5 hex = 45-51 chars
        assert!(sign.len() >= 45 && sign.len() <= 51);
    }

    #[test]
    fn test_random_guid() {
        let guid = random_guid();
        assert_eq!(guid.len(), 9);
        assert!(guid.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_detect_qualities_flac() {
        let file = FileInfo {
            media_mid: Some("abc".into()),
            size_128mp3: Some(3000000),
            size_320mp3: Some(9000000),
            size_flac: Some(25000000),
            size_ape: None,
            size_192aac: Some(5000000),
            size_hires: None,
        };
        let q = QQMusicApi::detect_qualities(&file);
        assert!(q.contains(&AudioQuality::Shq));
        assert!(q.contains(&AudioQuality::Hq));
        assert!(q.contains(&AudioQuality::Sq));
        assert!(q.contains(&AudioQuality::Lq));
    }

    #[test]
    fn test_detect_qualities_128_only() {
        let file = FileInfo {
            media_mid: Some("abc".into()),
            size_128mp3: Some(3000000),
            size_320mp3: None,
            size_flac: None,
            size_ape: None,
            size_192aac: None,
            size_hires: None,
        };
        let q = QQMusicApi::detect_qualities(&file);
        assert_eq!(q, vec![AudioQuality::Lq]);
    }
}
