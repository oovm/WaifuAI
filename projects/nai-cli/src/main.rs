use std::{
    fs::{create_dir, read_to_string},
    io::Write,
    path::PathBuf,
};
use std::future::Future;

use futures_util::StreamExt;
use rand::Rng;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};
use toml::from_str;

use clap::Args;
use clap::Parser;
use clap::Subcommand;
use novel_ai::{ImageRequest, ImageRequestBuilder, NaiError, NaiResult, NaiSecret};

pub mod builtin;
pub mod with_tags;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct NaiApp {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 生成新的常规图片
    New(CommandArgs),
    /// 生成新的涩涩图片
    SS(CommandArgs),
}

#[derive(Args, Debug)]
pub struct CommandArgs {
    /// 文件夹名
    #[arg(default_value_t = String::new())]
    name: String,
    /// 提示词
    #[arg(default_value_t = String::new())]
    tags: String,
    /// 开多少线程同时工作, 默认 3 个
    #[arg(short, long, default_value_t = 3)]
    threads: u8,
    /// 生成多少组图片, 默认 5 组
    #[arg(short, long, default_value_t = 5)]
    number: u32,
    /// 每组图片生成几张, 默认 1 张
    #[arg(short, long, default_value_t = 1)]
    frame: u8,
    /// 每组中图片的变化量, 默认 16
    #[arg(short, long, default_value_t = 16)]
    step: u32,
}

impl Commands {
    pub async fn  run(&self, cfg: &NaiConfig) -> NaiResult {
        let tasks = match self {
            Commands::New(args) => {args.prepare_tasks(&cfg.nai)}
            Commands::SS(args) => {args.prepare_tasks(&cfg.nai)}
        };
        let mut stream = tokio_stream::iter(tasks).buffer_unordered(threads);
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NaiConfig {
    nai: NaiSecret,
}

impl NaiConfig {
    pub fn load() -> NaiResult<Self> {
        match from_str(&read_to_string("nai.toml")?) {
            Ok(o) => Ok(o),
            Err(e) => return Err(NaiError::ParseError(e.to_string())),
        }
    }
}

impl CommandArgs {
    pub fn prepare_tasks(&self, secret: &NaiSecret) -> Vec<impl Future<Output=NaiResult>> {
        let mut tasks = Vec::new();
        for _ in 1..=self.number {
            let mut rng = thread_rng();
            let seed = rng.gen();
            let builder = TaskBuilder {
                tags: "best quality, masterpiece, highres, school uniform, devil, black hair, off_shoulder, {solo}".to_string(),
                seed,
                dir: PathBuf::from("target/nai/school uniform/"),
            };
            builder.ensure_path()?;
            for i in 0..=(self.frame - 1) {
                tasks.push(builder.clone().task(i * self.step, secret.nai))
            }
        }
        return tasks;
    }
}

#[tokio::main]
async fn main() -> NaiResult {
    let args = NaiApp::parse();
    let config =  NaiConfig::load()?;
    args.command.run(&config).await?;
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
