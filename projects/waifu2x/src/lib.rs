#![feature(once_cell)]

mod errors;
mod esr_gan;
pub mod nai_image;
mod secret;
mod sr_cnn;
mod sr_res;

pub use self::{
    errors::{Waifu2xError, Waifu2xResult},
    nai_image::{
        image_request::{ImageParameters, ImageRequest},
        ImageRequestBuilder,
    },
    secret::NaiSecret,
};
