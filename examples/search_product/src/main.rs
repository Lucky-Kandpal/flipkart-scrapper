use flipkart_scraper::ProductSearch;
use flipkart_scraper::search::{SortOrder, SearchParams};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = "samsung washing machine";
    let params = SearchParams {
        page_number: Some(1),
        sort: Some(SortOrder::PriceLowToHigh),
        min_price: Some(10000),
        max_price: Some(20000),
        others: None,
    };

    let details = ProductSearch::search(query.into(), params).await;

    if let Ok(s) = details {
        println!("{:#?}\n\nTotal {} search results.", s, s.results.len());
    } else {
        println!("{}", details.unwrap_err());
    }
    
    Ok(())
}
