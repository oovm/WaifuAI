use std::{env::current_exe, fs::create_dir_all, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use novel_ai::{ImageRequest, ImageRequestBuilder, NaiError, NaiResult, NaiSecret};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskBuilder {
    tags: String,
    seed: u32,
    dir_name: PathBuf,
}

impl TaskBuilder {
    pub fn new(tags: &str, file: &str, seed: u32) -> NaiResult<TaskBuilder> {
        let exe = current_exe()?.parent().unwrap().canonicalize()?;
        let task = TaskBuilder { tags: tags.to_string(), seed, dir_name: exe.join(format!("target/nai/{}/", file)) };
        task.ensure_path()?;
        Ok(task)
    }
    fn request(&self) -> ImageRequest {
        let mut builder = ImageRequestBuilder::default();
        builder.add_tag_split(&self.tags);
        builder.build()
    }
    pub fn ensure_path(&self) -> NaiResult {
        if !self.dir_name.exists() {
            create_dir_all(&self.dir_name)?;
        }
        let config = self.dir_name.join("_.toml");
        let mut file = std::fs::File::create(config)?;
        match toml::to_string(&self) {
            Ok(s) => file.write_all(s.as_bytes())?,
            Err(e) => return Err(NaiError::ParseError(e.to_string())),
        }
        Ok(())
    }
    async fn addition_mode(self) -> NaiResult {
        println!("追加模式",);
        Ok(())
    }

    pub async fn task(self, i: u32, nai: NaiSecret) -> NaiResult {
        if self.dir_name.exists() {
            // return self.addition_mode();
        }
        let mut request = self.request();
        let idx = 100 + i;
        request.parameters.noise = 0.2;
        request.parameters.seed = self.seed;
        request.parameters.scale = (idx as f32 / 10.0) - 0.0;
        // request.model = "safe-diffusion".to_string();
        let file_name = format!("{}-{}.png", request.parameters.seed, idx);
        println!("Draw:   {}", file_name);
        let mut file = File::create(self.dir_name.join(&file_name)).await?;
        let image = request.request_image(&nai).await?;
        file.write_all(&image).await?;
        println!("Finish: {}", file_name);
        Ok(())
    }
}
