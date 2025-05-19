use scraper::{Html, Selector};
use url::Url;

use super::errors::SearchError;
use super::SearchParams;
#[cfg(feature = "fetch")]
use crate::product_details::errors::FetchError;
#[cfg(feature = "fetch")]
use crate::ProductDetails;

#[cfg(feature = "fetch")]
use reqwest::Client;

#[cfg(feature = "wasm_parser")]
use tsify::Tsify;
#[cfg(feature = "wasm_parser")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm_parser", derive(Tsify))]
#[derive(Debug, Default)]
/// Product found in search results
pub struct SearchResult {
    /// Name of the product
    pub product_name: String,
    /// Link to the product
    pub product_link: String,
    /// URL to the thumbnail of the product
    pub thumbnail: String,
    #[cfg_attr(feature = "wasm_parser", tsify(optional))]
    /// Current price of the product
    pub current_price: Option<i32>,
    #[cfg_attr(feature = "wasm_parser", tsify(optional))]
    /// Original price of the product
    pub original_price: Option<i32>,
}

impl SearchResult {
    #[cfg(feature = "fetch")]
    /// Get detailed information about the searched product.
    pub async fn fetch_product(&self) -> Result<ProductDetails, FetchError> {
        let product_link = Url::parse(&self.product_link)
            .map_err(|source| FetchError::UrlParseError { source })?;
        ProductDetails::fetch(product_link).await
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
/// Search result of a product on Flipkart.
///
/// Use `ProductSearch::search` method to get the search results
pub struct ProductSearch {
    /// Original query used to search
    pub query: String,
    /// Query parameters used to search
    pub query_params: SearchParams,
    /// URL of the search query
    pub query_url: String,
    /// List of search results
    pub results: Vec<SearchResult>,
}

impl std::ops::Deref for ProductSearch {
    type Target = Vec<SearchResult>;
    fn deref(&self) -> &Self::Target {
        &self.results
    }
}
impl std::ops::DerefMut for ProductSearch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.results
    }
}

impl ProductSearch {
    /// Parses the HTML body and returns the search results.
    pub fn parse(webpage_content: String) -> Vec<SearchResult> {
        let div_selector = &Selector::parse("div").unwrap();
        let img_selector = &Selector::parse("img").unwrap();
        let link_selector = &Selector::parse("a").unwrap();

        let document = Html::parse_document(&webpage_content);

        let search_results = document
            .select(div_selector)
            .filter(|div| div.value().attr("data-id").is_some())
            .filter_map(|product| {
                let mut link_iter = product.select(link_selector);
                let mut link_elem = link_iter.next()?;
                let product_link: String = link_elem.value().attr("href").map(|link| {
                    if link.starts_with('/') {
                        String::from("https://flipkart.com") + link
                    } else {
                        link.into()
                    }
                })?;
                let thumbnail = link_elem
                    .select(img_selector)
                    .next()
                    .and_then(|img| img.value().attr("src"))?;

                let name_section = link_elem.last_child()?.value().as_element()?.classes();
                // select using the selector of classes
                let class_selector = &Selector::parse(
                    &name_section
                        .map(|sel| String::from('.') + sel)
                        .collect::<String>(),
                )
                .ok()?;
                let name = link_elem
                    .select(class_selector)
                    .next()
                    .and_then(|name_elem| {
                        let name = name_elem.text().next();
                        if name == Some("Sponsored") {
                            name_elem.text().nth(1)
                        } else {
                            name
                        }
                    })
                    .or_else(|| {
                        link_elem = link_iter.next()?;
                        link_elem.value().attr("title")
                    })
                    .or_else(|| link_elem.text().next())?;

                let mut current_price = None;
                let mut original_price = None;
                for div in product.select(div_selector) {
                    if let Some(price_tag) = div.text().next() {
                        if price_tag.starts_with('₹') {
                            let price_tag = div.text().collect::<String>();
                            let price_tag = price_tag.strip_prefix('₹').unwrap();
                            if price_tag.contains('₹') {
                                continue;
                            }
                            let price = price_tag.replace(',', "");
                            if current_price.is_none() {
                                current_price = price.parse::<i32>().ok();
                            } else {
                                original_price = price.parse::<i32>().ok();
                                break;
                            }
                        }
                    }
                }

                Some(SearchResult {
                    product_name: name.into(),
                    product_link,
                    thumbnail: thumbnail.into(),
                    current_price,
                    original_price,
                })
            })
            .collect::<Vec<_>>();

        search_results
    }

    pub fn build_request_url(query: String, params: SearchParams) -> Result<Url, SearchError> {
        let mut url_params = params.url_params();
        url_params.push(("q", query.clone()));

        Url::parse_with_params(
            "https://www.flipkart.com/search?marketplace=FLIPKART",
            url_params,
        )
        .map_err(|source| SearchError::UrlParseError { source })
    }

    #[cfg(feature = "fetch")]
    /// Searchs the query for a product on Flipkart.
    pub async fn search(query: String, params: SearchParams) -> Result<Self, SearchError> {
        let search_url = Self::build_request_url(query, params)?;

        let client = Client::builder()
            .default_headers(crate::build_headers())
            .build()
            .map_err(|source| SearchError::ClientBuilderError { source })?;

        let webpage = client
            .get(search_url.to_owned())
            .send()
            .await
            .map_err(|source| SearchError::HttpRequestError { source })?;

        let body = webpage
            .text()
            .await
            .map_err(|source| SearchError::WebpageTextParseError { source })?;

        let retry_error = body.contains("Retry in ");
        if retry_error {
            return Err(SearchError::FlipkartRetryError);
        }

        let search_results = Self::parse(body);

        Ok(ProductSearch {
            query: query,
            query_params: params,
            query_url: search_url.into(),
            results: search_results,
        })
    }
}
