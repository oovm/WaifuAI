#![feature(once_cell)]

mod errors;
mod esr_gan;
mod secret;
mod sr_cnn;
mod sr_res;
pub mod utils;

pub use self::{
    errors::{Waifu2xError, Waifu2xResult},
    secret::NaiSecret,
};
