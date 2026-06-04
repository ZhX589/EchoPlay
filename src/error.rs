use thiserror::Error;

#[derive(Debug, Error)]
pub enum EchoError {
    #[error("provider I/O error: {message}")]
    ProviderIO {
        message: String,
        #[source]
        source: Option<anyhow::Error>,
    },

    #[error("model not found: {reason:?}")]
    ModelNotFound { reason: NotFoundReason },

    #[error("media not found: {reason:?}")]
    MediaNotFound { reason: MediaNotFoundReason },

    #[error("provider '{0}' not found")]
    ProviderNotFound(String),

    #[error("provider '{0}' already registered")]
    ProviderAlreadyRegistered(String),

    #[error("no user logged in")]
    NoUserLoggedIn,

    #[error("URI parse error: {0}")]
    UriParseError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotFoundReason {
    NotFound,
    NotSupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaNotFoundReason {
    NotFound,
    CheckChildren,
}
