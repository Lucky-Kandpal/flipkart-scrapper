import * as flipkart_scraper from "./pkg/flipkart_scraper.js";


const url = "https://www.flipkart.com/search?q=asus%20vivobook";
const webpage = await fetch(url).then(async (response) => await response.text());

const data = flipkart_scraper.parse_search_results(webpage);
console.log(data);