use futures_util::StreamExt;
use novel_ai::{ImageRequest, ImageRequestBuilder, NaiError, NaiResult, NaiSecret};
use rand::thread_rng;
use std::{
    collections::VecDeque,
    fs::{create_dir, read_to_string},
    io::Write,
    path::PathBuf,
};
use tokio::{fs::File, io::AsyncWriteExt};
use toml::from_str;

pub mod with_tags;

use rand::Rng;

#[tokio::main]
async fn main() -> NaiResult {
    let nai: NaiSecret = match from_str(&read_to_string("nai.toml")?) {
        Ok(o) => o,
        Err(e) => return Err(NaiError::ParseError(e.to_string())),
    };
    // nai miko -f 5
    let concurrency = 3;
    let seeds = 5;
    let frame = 7;
    let step = 13;
    let mut tasks = VecDeque::new();

    for _ in 1..=seeds {
        let mut rng = thread_rng();
        let seed = rng.gen();
        let builder = TaskBuilder {
            tags: "best quality, masterpiece, detailed, 2girls, kiss".to_string(),
            seed,
            dir: PathBuf::from("target/nai/black red/"),
        };
        builder.ensure_path()?;
        for i in 0..=(frame - 1) {
            if i % 2 == 0 {
                tasks.push_back(builder.clone().task(i * step, &nai))
            }
            else {
                tasks.push_front(builder.clone().task(i * step, &nai))
            }
        }
    }
    let mut stream = tokio_stream::iter(tasks).buffer_unordered(concurrency);
    while let Some(task) = stream.next().await {
        match task {
            Ok(_) => {}
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }
    Ok(())
}
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskBuilder {
    tags: String,
    seed: u32,
    dir: PathBuf,
}

impl TaskBuilder {
    fn request(&self) -> ImageRequest {
        let mut builder = ImageRequestBuilder::default();
        builder.add_tag_split(&self.tags);
        builder.build()
    }
    fn ensure_path(&self) -> NaiResult {
        if !self.dir.exists() {
            create_dir(&self.dir)?;
            let config = self.dir.join("seed.toml");
            let mut file = std::fs::File::create(config)?;
            match toml::to_string(&self) {
                Ok(s) => file.write_all(s.as_bytes())?,
                Err(e) => return Err(NaiError::ParseError(e.to_string())),
            }
        }
        Ok(())
    }
    async fn task(self, i: u32, nai: &NaiSecret) -> NaiResult {
        let mut request = self.request();
        let idx = 100 + i;
        request.parameters.noise = 0.2;
        request.parameters.seed = self.seed;
        request.parameters.scale = (idx as f32 / 10.0) - 0.3;
        // request.model = "safe-diffusion".to_string();
        let file_name = format!("{}-{}.png", request.parameters.seed, idx);
        println!("Drawing {}", file_name);
        let file_path = self.dir.join(&file_name);
        if file_path.exists() {
            return Ok(());
        }
        let mut file = File::create(file_path).await?;
        let image = request.request_image(nai).await?;
        file.write_all(&image).await?;
        println!("Finish {}", file_name);
        Ok(())
    }
}
