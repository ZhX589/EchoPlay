use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lyric {
    pub identifier: String,
    pub source: String,
    /// Original lyric content (LRC format or plain text).
    pub content: String,
    /// Translated lyric content (if available).
    pub trans_content: String,
}
