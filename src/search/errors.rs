/// Represents the errors that can occur during the search process.
pub enum SearchError {
    /// URL parsing error.
    UrlParseError { source: url::ParseError },
    #[cfg(feature = "fetch")]
    /// Retry error from the server.
    FlipkartRetryError,
    #[cfg(feature = "fetch")]
    /// Request client build error.
    ClientBuilderError { source: reqwest::Error },
    #[cfg(feature = "fetch")]
    /// HTTP Request error.
    HttpRequestError { source: reqwest::Error },
    #[cfg(feature = "fetch")]
    /// Webpage body text parsing error.
    WebpageTextParseError { source: reqwest::Error },
}

