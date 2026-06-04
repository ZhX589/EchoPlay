use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, Serialize, Deserialize)]
pub enum AudioQuality {
    /// >320kbps, lossless
    #[strum(serialize = "shq")]
    Shq,
    /// ~320kbps, high quality
    #[strum(serialize = "hq")]
    Hq,
    /// ~200kbps, standard
    #[strum(serialize = "sq")]
    Sq,
    /// ~100kbps, low
    #[strum(serialize = "lq")]
    Lq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, Serialize, Deserialize)]
pub enum VideoQuality {
    #[strum(serialize = "fhd")]
    Fhd,
    #[strum(serialize = "hd")]
    Hd,
    #[strum(serialize = "sd")]
    Sd,
    #[strum(serialize = "ld")]
    Ld,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
pub enum MediaType {
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "image")]
    Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub url: String,
    pub media_type: MediaType,
    pub http_headers: HashMap<String, String>,
}

impl Media {
    pub fn audio(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            media_type: MediaType::Audio,
            http_headers: HashMap::new(),
        }
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.http_headers = headers;
        self
    }
}

/// Quality sort policy.
///
/// Parses policy strings like "hq<>", ">>>", "shq><":
/// - Letters before the bracket: preferred quality anchor
/// - `<>` means start from anchor, expand outward (both directions)
/// - `>` means start from anchor, prefer higher
/// - `<` means start from anchor, prefer lower
///
/// Default policy "hq<>" means: try hq first, then sq, then shq, then lq.
pub struct QualitySortPolicy;

impl QualitySortPolicy {
    pub fn apply_audio(policy: &str) -> Vec<AudioQuality> {
        let all = AudioQuality::iter().collect::<Vec<_>>();
        if policy.is_empty() {
            return all;
        }

        // Parse anchor quality from the beginning of the policy string
        let anchor_str: String = policy.chars().take_while(|c| c.is_alphabetic()).collect();
        let anchor = AudioQuality::iter().find(|q| q.to_string() == anchor_str);

        let anchor_idx = match anchor {
            Some(a) => all.iter().position(|q| *q == a).unwrap_or(0),
            None => 0,
        };

        let direction: &str = policy.trim_start_matches(|c: char| c.is_alphabetic());

        match direction {
            // "<>" : expand outward from anchor
            "<>" => {
                let mut result = vec![all[anchor_idx]];
                let mut lo = anchor_idx as isize - 1;
                let mut hi = anchor_idx + 1;
                let len = all.len() as isize;
                while lo >= 0 || hi < len as usize {
                    if hi < len as usize {
                        result.push(all[hi]);
                        hi += 1;
                    }
                    if lo >= 0 {
                        result.push(all[lo as usize]);
                        lo -= 1;
                    }
                }
                result
            }
            // ">" : prefer higher quality (toward index 0)
            ">" | "<" => {
                let mut result = Vec::new();
                if direction == ">" {
                    // higher quality = lower index
                    for q in all.iter().take(anchor_idx + 1).rev() {
                        result.push(*q);
                    }
                    for q in all.iter().skip(anchor_idx + 1) {
                        result.push(*q);
                    }
                } else {
                    // lower quality = higher index
                    for q in all.iter().skip(anchor_idx) {
                        result.push(*q);
                    }
                    for q in all.iter().take(anchor_idx).rev() {
                        result.push(*q);
                    }
                }
                result
            }
            _ => all,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_sort_default() {
        let sorted = QualitySortPolicy::apply_audio("hq<>");
        assert_eq!(sorted[0], AudioQuality::Hq);
        // expand outward: sq, then shq, then lq
        assert_eq!(sorted[1], AudioQuality::Sq);
    }

    #[test]
    fn test_quality_sort_higher() {
        let sorted = QualitySortPolicy::apply_audio("sq>");
        assert_eq!(sorted[0], AudioQuality::Sq);
        assert_eq!(sorted[1], AudioQuality::Hq);
        assert_eq!(sorted[2], AudioQuality::Shq);
    }

    #[test]
    fn test_quality_sort_lower() {
        let sorted = QualitySortPolicy::apply_audio("sq<");
        assert_eq!(sorted[0], AudioQuality::Sq);
        assert_eq!(sorted[1], AudioQuality::Lq);
    }

    #[test]
    fn test_media_audio_constructor() {
        let m = Media::audio("https://example.com/song.mp3");
        assert_eq!(m.url, "https://example.com/song.mp3");
        assert_eq!(m.media_type, MediaType::Audio);
        assert!(m.http_headers.is_empty());
    }
}
