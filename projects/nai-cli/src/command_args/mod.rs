use std::{
    fs::{create_dir, read_to_string},
    io::Write,
    path::PathBuf,
};
use std::collections::hash_map::DefaultHasher;
use std::future::Future;
use std::hash::{Hash, Hasher};

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

impl CommandArgs {
    pub fn prepare_tasks(mut self, secret: NaiSecret, kind: &str) -> Vec<impl Future<Output=NaiResult>> {
        let mut tasks = Vec::new();
        if self.tags.is_empty() {
            self.tags = match kind {
                "ss" => { BuiltinPrompt::se_se() }
                _ => BuiltinPrompt::normal()
            }
        }
        if self.name.is_empty() {
            let mut hasher = DefaultHasher::new();
            self.tags.hash(&mut hasher);
            self.name = hasher.finish().to_string();
        }
        for _ in 1..=self.number {
            let mut rng = thread_rng();
            let seed = rng.gen();
            let builder = TaskBuilder {
                tags: self.tags.clone(),
                seed,
                dir: PathBuf::from(format!("target/nai/{}/", &self.name)),
            };
            match builder.ensure_path() {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    return tasks;
                }
            }
            for i in 0..=(self.frame - 1) {
                tasks.push(builder.clone().task(i * self.step, secret.clone()))
            }
        }
        return tasks;
    }
}

impl Commands {
    pub async fn run(&self, cfg: &NaiConfig) -> NaiResult {
        let mut threads = 3;
        let tasks = match self {
            Commands::New(args) => {
                threads = args.threads;
                args.prepare_tasks(cfg.nai.clone(), "new")
            }
            Commands::SS(args) => {
                threads = args.threads;
                args.prepare_tasks(cfg.nai.clone(), "ss")
            }
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
