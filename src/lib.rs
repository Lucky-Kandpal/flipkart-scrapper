//! Scrape flipkart product details and search results.
//!
//! `ProductDetails` can fetch details from product's URL
//! and `ProductSearch` can search a product from a
//! given search query from Flipkart.
//!
//! Feature Flags:
//! - `serde`: Enables serde support for the structs. (default)
//! - `fetch`: Enables fetching product details from the URL.

pub mod product_details;
pub mod search;

#[cfg(feature = "fetch")]
use header::{HeaderMap, HeaderValue};
pub use product_details::ProductDetails;
#[cfg(feature = "fetch")]
use reqwest::header;
pub use search::ProductSearch;
pub use url::Url;
#[cfg(feature = "wasm_parser")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "fetch")]
/// Builds the default headers for the client.
fn build_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/118.0",
        ),
    );
    headers.insert(
        header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.5"),
    );
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
    );
    headers
}

#[cfg(feature = "wasm_parser")]
/// Parses the HTML body and returns the search results.
#[wasm_bindgen]
pub fn parse_search_results(webpage_body: JsValue) -> Result<JsValue, JsError> {
    let body = webpage_body
        .as_string()
        .ok_or(JsError::new("Argument passed is not a valid string"))?;

    let search_results = ProductSearch::parse(body);

    Ok(serde_wasm_bindgen::to_value(&search_results)?)
}
