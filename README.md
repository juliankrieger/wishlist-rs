# Amazon Wishlist Scraper

This project is a small web scraper for Amazon wishlists. Since Amazon sadly decided to shut down their wishlist API a couple of years ago, `wishlist-rs` exists to fill the gap until Amazon decides to make its API public again.

## How does it work?

The project is split into `lib` and `bin`: You can download and consume the library functions to further work with `wishlist-rs` or just install it as a binary. 

## Usage

Upon execution, the binary runs a small webserver on port 8080. You can then access it with your Amazon wishlist URL as a parameter to get back json data which describes each wishlist item. 

Example:
```
http://localhost:8080/https%3A%2F%2Fwww.amazon.de%2F-%2Fen%2Fhz%2Fwishlist%2Fls%2<wishlist_id>

Returns: 

[
   {
      "image_src":"https://m.media-amazon.com/images/I/410UXAx8inL._SS135_.jpg",
      "name":"On Killing: The Psychological Cost of Learning to Kill in War and Society",
      "info":"{\"hasComparisonTable\":false,\"itemExternalId\":\"ASIN:0316040932|A1PA6795UKMFR9\",\"listType\":\"WishList\",\"sid\":\"xxxxxx\"}",
      "link":"www.amazon.de/-/en/dp/0316040932/?coliid=I1K0C1COWLBC9U&colid=DI5WT15AHPLM&psc=1&ref_=lv_vv_lig_dp_it",
      "price":"17.0",
      "byline":"by Lieutenant Colonel Dave Grossman (Paperback)"
   },
   ...
]
```

## Notes

### Why does this not make use of Amazon's ASIN?

To request item information via [their Product Advertising API](https://webservices.amazon.com/paapi5/documentation/get-items.html), the user would need a P-API account. These are pretty hard to come by and not given out to just anybody by Amazon.

### How high are the chances this will break in the future?

Pretty high. The problem with using webscrapers is that they are fickle to use and break easily if Amazon ever decides to change the HTML they deliver.

### Why is this slow?

For performance reasons, Amazon decided to use lazy loading in their wishlists. A wishlist only loads its first 10 entries. To circumvent this issue, `wishlist-rs`'s webscraper scrolls down to the bottom of the page and waits for Amazon's `endOfList` element to load.

## Future Plans

- Implement arguments for the binary, specifying `webdriver_port`, `webservice_port` and logging
- Documentation
- Build a more robust scraper
- Enable `unparsed` version where the webservice just returns unparsed HTML for each wishlist item
- add tests

## License

This project is licensed under the MIT license.