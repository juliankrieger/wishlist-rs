use actix_web::{get, web::{self, Data}, App, HttpServer};
use wishlist_rs::{get_wishlist_items, check_valid_wishlist_url};
mod error;
use error::WishlistError;

struct AppState {
    webdriver_port: String,
}

#[get("/{wishlist_url}")]
async fn index(params: web::Path<String>, data: Data<AppState>) -> Result<String, WishlistError> {
    let wishlist_url = params.into_inner();
    let is_url_valid = check_valid_wishlist_url(&wishlist_url);

    if is_url_valid.is_err() {
        return Err(WishlistError::BadClientData);
    }

    let webdriver_port = &data.webdriver_port;

    let wishlist_items = get_wishlist_items(&wishlist_url, webdriver_port).await;

    if let Ok(serializable_items) = wishlist_items {
        let serialized = serde_json::to_string(&serializable_items).unwrap();
        return Ok(serialized);
    } else {
        let error = wishlist_items.unwrap_err();
        print!("{}", error);
        return Err(WishlistError::InternalError);
    }
}

// let's set up the sequence of steps we want the browser to take
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data = Data::new(AppState {
        webdriver_port: String::from("4444"),
    });

    HttpServer::new(move || 
        App::new()
        .app_data(Data::clone(&data))
        .service(index)
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
