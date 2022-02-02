use actix_web::error;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum WishlistError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,
}

impl error::ResponseError for WishlistError {}
