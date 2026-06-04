use crate::model::BriefSong;

/// Scored standby candidate from another provider.
#[derive(Debug, Clone)]
pub struct StandbyCandidate {
    pub song: BriefSong,
    pub score: f64,
}

/// Cross-provider standby matcher.
///
/// When a song's media is unavailable from its original provider, this matcher
/// searches other providers and scores candidates by similarity.
///
/// Scoring (0.0 - 1.0):
/// - Title match: 0.4 weight
/// - Artists name match: 0.3 weight
/// - Album name match: 0.2 weight
/// - Duration match: 0.1 weight
pub struct StandbyMatcher {
    min_score: f64,
}

impl StandbyMatcher {
    pub fn new(min_score: f64) -> Self {
        Self { min_score }
    }

    /// Score a candidate song against the original.
    pub fn score(original: &BriefSong, candidate: &BriefSong) -> f64 {
        let title_score = Self::similarity(&original.title, &candidate.title);
        let artist_score = Self::similarity(&original.artists_name, &candidate.artists_name);
        let album_score = Self::similarity(&original.album_name, &candidate.album_name);
        let duration_score = Self::duration_match(&original.duration_ms, &candidate.duration_ms);

        title_score * 0.4 + artist_score * 0.3 + album_score * 0.2 + duration_score * 0.1
    }

    /// Filter and rank candidates above the minimum score threshold.
    pub fn filter(
        &self,
        original: &BriefSong,
        candidates: Vec<BriefSong>,
    ) -> Vec<StandbyCandidate> {
        let mut results: Vec<StandbyCandidate> = candidates
            .into_iter()
            .filter_map(|c| {
                let score = Self::score(original, &c);
                if score >= self.min_score {
                    Some(StandbyCandidate { song: c, score })
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// Simple string similarity: ratio of matching characters (0.0 - 1.0).
    fn similarity(a: &str, b: &str) -> f64 {
        if a == b {
            return 1.0;
        }
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }

        // Normalize: lowercase, remove whitespace
        let norm_a: String = a
            .to_lowercase()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let norm_b: String = b
            .to_lowercase()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        if norm_a == norm_b {
            return 1.0;
        }

        // Count matching characters (simple approach)
        let mut matches = 0;
        let mut remaining: Vec<char> = norm_b.chars().collect();
        for ch in norm_a.chars() {
            if let Some(pos) = remaining.iter().position(|&c| c == ch) {
                remaining.remove(pos);
                matches += 1;
            }
        }

        let max_len = norm_a.len().max(norm_b.len()) as f64;
        matches as f64 / max_len
    }

    /// Duration similarity: returns 1.0 if within 5 seconds, degrades linearly.
    fn duration_match(a_ms: &str, b_ms: &str) -> f64 {
        let a: i64 = a_ms.parse().unwrap_or(0);
        let b: i64 = b_ms.parse().unwrap_or(0);

        if a == 0 || b == 0 {
            return 0.5; // unknown duration, neutral score
        }

        let diff = (a - b).unsigned_abs() as f64;
        let tolerance = 5000.0; // 5 seconds

        if diff <= tolerance {
            1.0
        } else {
            // Decay: 0 at 30 seconds difference
            let decay = 1.0 - (diff - tolerance) / 25000.0;
            decay.max(0.0)
        }
    }
}

impl Default for StandbyMatcher {
    fn default() -> Self {
        Self::new(0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_song(title: &str, artist: &str, album: &str, duration: &str) -> BriefSong {
        BriefSong {
            identifier: "1".into(),
            source: "test".into(),
            title: title.into(),
            artists_name: artist.into(),
            album_name: album.into(),
            duration_ms: duration.into(),
        }
    }

    #[test]
    fn test_identical_songs_score_1() {
        let a = make_song("Hello", "World", "Album", "240000");
        let b = make_song("Hello", "World", "Album", "240000");
        assert!((StandbyMatcher::score(&a, &b) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_completely_different_songs() {
        let a = make_song("Hello", "World", "Album", "240000");
        let b = make_song("XYZ", "ABC", "Other", "180000");
        assert!(StandbyMatcher::score(&a, &b) < 0.3);
    }

    #[test]
    fn test_same_title_different_artist() {
        let a = make_song("Hello", "Adele", "25", "295000");
        let b = make_song("Hello", "Someone Else", "Other", "295000");
        let score = StandbyMatcher::score(&a, &b);
        assert!(score > 0.4 && score < 0.8);
    }

    #[test]
    fn test_filter_above_threshold() {
        let original = make_song("Hello", "Adele", "25", "295000");
        let matcher = StandbyMatcher::new(0.5);

        let candidates = vec![
            make_song("Hello", "Adele", "25", "295000"), // identical
            make_song("Hello", "Adele", "Other", "295000"), // same title+artist
            make_song("XYZ", "ABC", "DEF", "100000"),    // totally different
        ];

        let results = matcher.filter(&original, candidates);
        assert!(results.len() >= 2);
        // First result should have highest score
        assert!(results[0].score >= results[1].score);
    }

    #[test]
    fn test_duration_match_identical() {
        assert!((StandbyMatcher::duration_match("240000", "240000") - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_duration_match_close() {
        assert!((StandbyMatcher::duration_match("240000", "243000") - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_duration_match_far_apart() {
        assert!(StandbyMatcher::duration_match("240000", "300000") < 0.5);
    }
}
