//! Scrape flipkart product details and search results.
//!
//! `ProductDetails` can fetch details from product's URL
//! and `ProductSearch` can search a product from a
//! given search query from Flipkart.
//!
//! Feature Flags:
//! - `serde`: Enables serde support for the structs. (default)
//! - `fetch`: Enables fetching product details from the URL.
//! - `wasm_parser`: Enables parsing the HTML Body parsing for nodejs by building wasm build for the parser API.

pub mod product_details;
pub mod search;

pub use product_details::ProductDetails;
pub use search::ProductSearch;
pub use url::Url;

#[cfg(feature = "wasm_parser")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "fetch")]
/// Builds the default headers for the client.
fn build_headers() -> reqwest::header::HeaderMap {
    use reqwest::header;
    use header::{HeaderMap, HeaderValue};
    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:138.0) Gecko/20100101 Firefox/138.0",
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
#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "wasm_parser", derive(tsify::Tsify), tsify(into_wasm_abi))]
/// Wrapper of the search results vector.
pub struct Search(Vec<search::SearchResult>);

#[cfg(feature = "wasm_parser")]
#[wasm_bindgen]
/// Parses the HTML body and returns the search results.
pub fn parse_search_results(webpage_body: String) -> Search {
    let search_results = ProductSearch::parse(webpage_body);
    Search(search_results)
}

#[cfg(feature = "wasm_parser")]
#[wasm_bindgen]
/// Parses the HTML body and returns the product details.
pub fn parse_product_details(webpage_body: String) -> Result<ProductDetails, JsError> {
    ProductDetails::parse(webpage_body).map_err(|e| JsError::new(&e.to_string()))
}
