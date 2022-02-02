use fantoccini::{ClientBuilder, Locator};
use serde::Serialize;
use url::{Url, ParseError};

#[derive(Debug, Serialize)]
pub struct WishlistItem {
    image_src: String,
    name: String,
    info: String,
    link: String,
    price: String,
    byline: String,
}

pub fn check_valid_wishlist_url<'a>(wishlist_url: &'a str) -> Result<(), ParseError> {
    let url = Url::parse(wishlist_url)?;

    assert!(url.host_str().unwrap().contains("amazon"));
    assert!(url.password() == None);
    assert!(url.username() == "");
    assert!(url.scheme() == "https");
    assert!(url.path().contains("wishlist"));

    Ok(())
}

// let's set up the sequence of steps we want the browser to take
pub async fn get_wishlist_items<'a>(wishlist_url: &'a str, webdriver_port: &'a str) -> Result<Vec<WishlistItem>, fantoccini::error::CmdError> {
    let webdriver_url = format!("http://localhost:{}", webdriver_port);
    let mut c = ClientBuilder::native().connect(&webdriver_url).await.expect("failed to connect to WebDriver");
    
    // first, go to the Wikipedia page for Foobar
    c.goto(wishlist_url).await?;

    let host = c.current_url().await?.host().unwrap().to_string();

    c.execute("window.scrollTo(0,document.body.scrollHeight);", vec![]).await?;

    c.wait().for_element(Locator::Css("#endOfListMarker")).await?;

    let elements = c.find_all(Locator::Css("#wl-item-view li.g-item-sortable")).await?;

    let mut wishlist_items: Vec<WishlistItem> = vec![];

    for mut elem in elements {
        let mut image = elem.find(Locator::Css(".a-link-normal > img")).await?;
        
        let image_src = image.attr("src").await?.unwrap_or_default();

        let name = image.attr("alt").await?.unwrap_or_default();

        let byline = elem.find(Locator::Css("span[id^=item-byline]")).await?.text().await?;

        let price = elem.attr("data-price").await?.unwrap_or_default();

        let info = elem.attr("data-reposition-action-params").await?.unwrap_or_default();

        let mut link_elem = elem.find(Locator::Css("a[id^=itemName]")).await?;
        let relative_link = link_elem.attr("href").await?.unwrap_or_default();       
        let link = format!("{}{}", host, relative_link);

        let wishlist_item = WishlistItem {
            image_src,
            name,
            byline,
            price,
            info,
            link
        };

        wishlist_items.push(wishlist_item);
    }

    c.close().await?;

    print!("{:#?}", wishlist_items);

    Ok(wishlist_items)
}