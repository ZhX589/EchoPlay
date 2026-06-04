use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BriefUser {
    pub identifier: String,
    pub source: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    // --- Brief fields ---
    pub identifier: String,
    pub source: String,
    pub name: String,
    // --- Normal fields ---
    pub avatar_url: String,
}

impl BriefUser {
    pub fn into_user(self) -> User {
        User {
            identifier: self.identifier,
            source: self.source,
            name: self.name,
            avatar_url: String::new(),
        }
    }
}
