use thiserror::Error;

/// Represents the errors that can occur during the search process.
#[derive(Error, Debug)]
pub enum SearchError {
    /// URL parsing error.
    #[error("URL parsing error")]
    UrlParseError { source: url::ParseError },
    #[cfg(feature = "fetch")]
    /// Retry error from the server.
    #[error("Retry error from the server")]
    FlipkartRetryError,
    #[cfg(feature = "fetch")]
    /// Request client build error.
    #[error("Request client build error")]
    ClientBuilderError { source: reqwest::Error },
    #[cfg(feature = "fetch")]
    /// HTTP Request error.
    #[error("HTTP Request error")]
    HttpRequestError { source: reqwest::Error },
    #[cfg(feature = "fetch")]
    /// Webpage body text parsing error.
    #[error("Webpage body text parsing error")]
    WebpageTextParseError { source: reqwest::Error },
}

