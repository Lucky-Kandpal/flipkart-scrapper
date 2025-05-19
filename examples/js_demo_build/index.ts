import flipkart_scraper from "flipkart_scraper";

const url = flipkart_scraper.build_search_url("asus vivovook", {
    max_price: 40000,
    min_price: undefined,
    page_number: 2,
    sort: "price_high_to_low",
    others: undefined,
});

console.log("Search URL:", url);

const webpage = await fetch(url).then(
    async (response) => await response.text()
);

const data = flipkart_scraper.parse_search_results(webpage);

console.log("Search results:", data);

if (data.length !== 0) {
    console.log("\nNow fetching details of the top search result product...\n");
    const product_data = data[0];

    const product_url = product_data.product_link;
    const product_webpage = await fetch(product_url, {
        credentials: "omit",
        method: "GET",
        mode: "cors",
    }).then(async (response) => await response.text());

    const product_details =
        flipkart_scraper.parse_product_details(product_webpage);

    console.log(product_details);
}
