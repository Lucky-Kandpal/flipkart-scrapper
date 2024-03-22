import flipkart_scraper from "@dvishal485/flipkart_scraper";
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
        "headers": {
            "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:123.0) Gecko/20100101 Firefox/123.0",
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
            "Accept-Language": "en-US,en;q=0.5",
            "Sec-GPC": "1",
            "Upgrade-Insecure-Requests": "1",
            "Sec-Fetch-Dest": "document",
            "Sec-Fetch-Mode": "navigate",
            "Sec-Fetch-Site": "none",
            "Sec-Fetch-User": "?1"
        },
        "method": "GET",
        "mode": "cors"
    }).then(async (response) => await response.text());
    const product_details = flipkart_scraper.parse_product_details("product_webpage");
    console.log(product_details);
}
