/// Application configuration.
///
/// Loaded from `~/.config/echoplay/config.toml` or defaults.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub audio: AudioConfig,
    pub library: LibraryConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioConfig {
    /// Volume 0-100.
    pub volume: u8,
    /// Audio device name ("default" for system default).
    pub device: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryConfig {
    /// Directories to scan for local music.
    pub scan_dirs: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiConfig {
    /// Theme name.
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio: AudioConfig {
                volume: 80,
                device: "default".into(),
            },
            library: LibraryConfig { scan_dirs: vec![] },
            ui: UiConfig {
                theme: "default".into(),
            },
        }
    }
}

impl Config {
    /// Load config from the default path, or return defaults if not found.
    /// Logs a warning if the file exists but cannot be parsed.
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<Config>(&content) {
                    Ok(config) => config,
                    Err(e) => {
                        tracing::warn!(
                            "Failed to parse config at {}: {}. Using defaults.",
                            path.display(),
                            e
                        );
                        Self::default()
                    }
                },
                Err(e) => {
                    tracing::warn!(
                        "Failed to read config at {}: {}. Using defaults.",
                        path.display(),
                        e
                    );
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }

    /// Save config to the default path.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                tracing::error!("Failed to create config directory: {}", e);
                e
            })?;
        }
        let content = toml::to_string_pretty(self).unwrap_or_default();
        std::fs::write(&path, content).map_err(|e| {
            tracing::error!("Failed to write config: {}", e);
            e
        })
    }

    /// Get the config file path: `~/.config/echoplay/config.toml`
    pub fn config_path() -> std::path::PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("echoplay")
            .join("config.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = Config::default();
        assert_eq!(cfg.audio.volume, 80);
        assert_eq!(cfg.audio.device, "default");
        assert!(cfg.library.scan_dirs.is_empty());
        assert_eq!(cfg.ui.theme, "default");
    }

    #[test]
    fn test_config_roundtrip() {
        let cfg = Config::default();
        let toml_str = toml::to_string(&cfg).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.audio.volume, cfg.audio.volume);
        assert_eq!(parsed.audio.device, cfg.audio.device);
    }

    #[test]
    fn test_load_nonexistent() {
        // Should return defaults without panicking
        let cfg = Config::load();
        assert_eq!(cfg.audio.volume, 80);
    }
}
