use std::{fs::create_dir, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use novel_ai::{ImageRequest, ImageRequestBuilder, NaiError, NaiResult, NaiSecret};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskBuilder {
    pub tags: String,
    pub seed: u32,
    pub dir: PathBuf,
}

impl TaskBuilder {
    fn request(&self) -> ImageRequest {
        let mut builder = ImageRequestBuilder::default();
        builder.add_tag_split(&self.tags);
        builder.build()
    }
    pub fn ensure_path(&self) -> NaiResult {
        if !self.dir.exists() {
            create_dir(&self.dir)?;
            let config = self.dir.join("_.toml");
            let mut file = std::fs::File::create(config)?;
            match toml::to_string(&self) {
                Ok(s) => file.write_all(s.as_bytes())?,
                Err(e) => return Err(NaiError::ParseError(e.to_string())),
            }
        }
        Ok(())
    }
    pub async fn task(self, i: u32, nai: NaiSecret) -> NaiResult {
        let mut request = self.request();
        let idx = 100 + i;
        request.parameters.noise = 0.2;
        request.parameters.seed = self.seed;
        request.parameters.scale = (idx as f32 / 10.0) - 0.0;
        // request.model = "safe-diffusion".to_string();
        let file_name = format!("{}-{}.png", request.parameters.seed, idx);
        println!("Draw:   {}", file_name);
        let file_path = self.dir.join(&file_name);
        if file_path.exists() {
            return Ok(());
        }
        let mut file = File::create(file_path).await?;
        let image = request.request_image(&nai).await?;
        file.write_all(&image).await?;
        println!("Finish: {}", file_name);
        Ok(())
    }
}
