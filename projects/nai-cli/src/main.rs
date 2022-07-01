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
use serde::{Deserialize, Serialize};
pub mod builtin;
pub mod with_tags;
use rand::Rng;


use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    threads: u8,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    seeds: u8,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    frame: u8,


}


#[tokio::main]
async fn main() -> NaiResult {
    let args = Args::parse();
    let nai: NaiSecret = match from_str(&read_to_string("nai.toml")?) {
        Ok(o) => o,
        Err(e) => return Err(NaiError::ParseError(e.to_string())),
    };
    let threads = 3;
    let seeds = 16;
    let frame = 1;
    let step = 16;
    let mut tasks = Vec::new();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    if false {
        for _ in 1..=seeds {
            let mut rng = thread_rng();
            let seed = rng.gen();
            let builder = TaskBuilder {
                tags: "best quality, masterpiece, highres, school uniform, devil, black hair, off_shoulder, {solo}".to_string(),
                seed,
                dir: PathBuf::from("target/nai/school uniform/"),
            };
            builder.ensure_path()?;
            for i in 0..=(frame - 1) {
                tasks.push(builder.clone().task(i * step, &nai))
            }
        }
        let mut stream = tokio_stream::iter(tasks).buffer_unordered(threads);
        while let Some(task) = stream.next().await {
            match task {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
    }


    Ok(())
}





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
        request.parameters.scale = (idx as f32 / 10.0) - 0.0;
        // request.model = "safe-diffusion".to_string();
        let file_name = format!("{}-{}.png", request.parameters.seed, idx);
        println!("Draw:   {}", file_name);
        let file_path = self.dir.join(&file_name);
        if file_path.exists() {
            return Ok(());
        }
        let mut file = File::create(file_path).await?;
        let image = request.request_image(nai).await?;
        file.write_all(&image).await?;
        println!("Finish: {}", file_name);
        Ok(())
    }
}
