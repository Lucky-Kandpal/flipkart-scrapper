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

#[cfg(feature = "fetch")]
use header::{HeaderMap, HeaderValue};
pub use product_details::ProductDetails;
#[cfg(feature = "fetch")]
use reqwest::header;
pub use search::ProductSearch;
pub use url::Url;
#[cfg(feature = "wasm_parser")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm_parser")]
use tsify::Tsify;

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
#[derive(Tsify, serde::Deserialize, serde::Serialize)]
#[tsify(into_wasm_abi)]
pub struct Search(Vec<search::SearchResult>);

#[cfg(feature = "wasm_parser")]
/// Parses the HTML body and returns the search results.
#[wasm_bindgen]
pub fn parse_search_results(webpage_body: String) -> Result<Search, JsError> {
    let search_results = ProductSearch::parse(webpage_body);

    // Ok(serde_wasm_bindgen::to_value(&search_results)?)
    Ok(Search(search_results))
}

#[cfg(feature = "wasm_parser")]
/// Parses the HTML body and returns the product details.
#[wasm_bindgen]
pub fn parse_product_details(webpage_body: String) -> Result<ProductDetails, JsError> {
    let product_details =
        ProductDetails::parse(webpage_body).map_err(|e| JsError::new(&e.to_string()))?;

    Ok(product_details)
}
