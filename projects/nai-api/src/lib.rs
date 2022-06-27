mod errors;
pub mod nai_image;
mod secret;
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NaiSecret {
    bearer: String,
}
