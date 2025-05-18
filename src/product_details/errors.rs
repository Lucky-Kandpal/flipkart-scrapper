#[derive(Debug, Clone, Copy, thiserror::Error)]
/// Represents the errors that can occur when trying to get product details
pub enum ProductDetailsError {
    /// Link provided doesn't corresponds to any product.
    #[error("Link provided doesn't corresponds to any product")]
    NonProductLink,
    /// Internal Server Error. Host is down or is blocking use of this library.
    #[error("Internal Server Error. Host is down or is blocking use of this library")]
    InternalServerError,
    /// Flipkart labelled the network request as a potential bot service.
    #[error("Flipkart labelled the network request as a potential bot service")]
    IdentifiedAsBot,
}

pub enum FetchError {
    /// Only flipkart.com is supported.
    NonFlipkartDomain,
    /// Domain name is invalid. Either URL doesn't have a host or is specified as an IP address.
    InvalidDomainName,
    /// Product details parsing error.
    ProductDetailsError { source: ProductDetailsError },
    /// URL parsing error.
    UrlParseError { source: url::ParseError },
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
