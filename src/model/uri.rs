use super::base::ModelType;
use crate::error::EchoError;

/// URI format: `echoplay://{source}/{model_type_plural}/{identifier}`
/// Example: `echoplay://qq/songs/12345`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri {
    pub source: String,
    pub model_type: ModelType,
    pub identifier: String,
}

impl Uri {
    pub fn new(
        source: impl Into<String>,
        model_type: ModelType,
        identifier: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            model_type,
            identifier: identifier.into(),
        }
    }

    pub fn parse(s: &str) -> Result<Self, EchoError> {
        // Accept both "echoplay://source/type/id" and "source/type/id"
        let s = s.strip_prefix("echoplay://").unwrap_or(s);

        let parts: Vec<&str> = s.splitn(3, '/').collect();
        if parts.len() != 3 {
            return Err(EchoError::UriParseError(format!(
                "expected 3 segments (source/type/id), got {} in '{}'",
                parts.len(),
                s
            )));
        }

        let source = parts[0].to_string();
        let model_type = ModelType::from_plural(parts[1]).ok_or_else(|| {
            EchoError::UriParseError(format!("unknown model type: '{}'", parts[1]))
        })?;
        let identifier = parts[2].to_string();

        Ok(Self {
            source,
            model_type,
            identifier,
        })
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "echoplay://{}/{}/{}",
            self.source,
            self.model_type.plural(),
            self.identifier
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full_uri() {
        let uri = Uri::parse("echoplay://qq/songs/12345").unwrap();
        assert_eq!(uri.source, "qq");
        assert_eq!(uri.model_type, ModelType::Song);
        assert_eq!(uri.identifier, "12345");
    }

    #[test]
    fn test_parse_short_uri() {
        let uri = Uri::parse("netease/albums/999").unwrap();
        assert_eq!(uri.source, "netease");
        assert_eq!(uri.model_type, ModelType::Album);
        assert_eq!(uri.identifier, "999");
    }

    #[test]
    fn test_parse_invalid_segments() {
        assert!(Uri::parse("qq/songs").is_err());
    }

    #[test]
    fn test_parse_unknown_type() {
        assert!(Uri::parse("qq/unknown/1").is_err());
    }

    #[test]
    fn test_display_roundtrip() {
        let uri = Uri::new("qq", ModelType::Song, "12345");
        let s = uri.to_string();
        assert_eq!(s, "echoplay://qq/songs/12345");
        let parsed = Uri::parse(&s).unwrap();
        assert_eq!(parsed, uri);
    }

    #[test]
    fn test_all_model_types_roundtrip() {
        for mt in [
            ModelType::Song,
            ModelType::Album,
            ModelType::Artist,
            ModelType::Playlist,
            ModelType::Lyric,
            ModelType::Video,
            ModelType::User,
        ] {
            let uri = Uri::new("test", mt, "id");
            let s = uri.to_string();
            let parsed = Uri::parse(&s).unwrap();
            assert_eq!(parsed.model_type, mt);
        }
    }
}
