import flipkart_scraper from "flipkart_scraper";
const url = "https://www.flipkart.com/search?q=asus%20vivobook";
const webpage = await fetch(url).then(async (response) => await response.text());
const data = flipkart_scraper.parse_search_results(webpage);
console.log("Search results:", data);
if (data.length !== 0) {
    console.log("\nNow fetching details of the top search result product...\n");
    const product_data = data[0];
    const product_url = product_data.product_link;
    const product_webpage = await fetch(product_url, {
        "credentials": "omit",
        "method": "GET",
        "mode": "cors"
    }).then(async (response) => await response.text());
    const product_details = flipkart_scraper.parse_product_details(product_webpage);
    console.log(product_details);
}
