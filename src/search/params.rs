use std::collections::HashMap;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
/// Parameters for the search request.
pub struct SearchParams {
    /// The page number to return.
    pub page_number: Option<u16>,
    /// The sort order of the results.
    pub sort: Option<SortOrder>,
    /// Minimum price range for the search.
    ///
    /// Flipkart rounds off or alters the range based on product.
    pub min_price: Option<u16>,
    /// Maximum price range for the search.
    ///
    /// Flipkart rounds off or alter the range based on product.
    pub max_price: Option<u16>,
    /// Other parameters to be passed in URL query param of search request.
    pub others: Option<HashMap<String, String>>,
}

impl SearchParams {
    pub fn url_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();

        if let Some(page_number) = self.page_number {
            params.push(("page", page_number.to_string()));
        }

        if let Some(sort) = &self.sort {
            let sort_value = match sort {
                SortOrder::Relevance => "relevance",
                SortOrder::PriceLowToHigh => "price_asc",
                SortOrder::PriceHighToLow => "price_desc",
                SortOrder::NewestFirst => "recency_desc",
                SortOrder::Popularity => "popularity",
            }
            .to_string();
            params.push(("sort", sort_value));
        }

        // https://www.flipkart.com/search?q=iphone&p%5B%5D=facets.price_range.from%3DMin&p%5B%5D=facets.price_range.to%3D10000
        let price_range = match (self.min_price, self.max_price) {
            (None, None) => None,
            (None, Some(max)) => Some(("Min".to_owned(), max.to_string())),
            (Some(min), None) => Some((min.to_string(), "Max".to_owned())),
            (Some(min), Some(max)) => Some((min.to_string(), max.to_string())),
        };

        if let Some((min_price, max_price)) = price_range {
            params.push(("p[]", format!("facets.price_range.from={min_price}")));
            params.push(("p[]", format!("facets.price_range.to={max_price}")));
        }

        if let Some(others) = &self.others {
            for (key, value) in others {
                params.push((key, value.to_owned()));
            }
        }

        params
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone)]
/// The sort order of the search results.
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    /// Sort by relevance.
    Relevance,
    /// Sort by price low to high.
    PriceLowToHigh,
    /// Sort by price high to low.
    PriceHighToLow,
    /// Sort by newest first.
    NewestFirst,
    /// Sort by popularity.
    Popularity,
}
