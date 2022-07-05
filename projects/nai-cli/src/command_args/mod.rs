use std::{future::Future, path::PathBuf};

use futures_util::StreamExt;
use rand::{thread_rng, Rng};

use novel_ai::{NaiResult, NaiSecret};

use crate::{task_builder::TaskBuilder, BuiltinPrompt, CommandArgs, Commands, NaiConfig};

impl CommandArgs {
    pub fn prepare_tasks(mut self, secret: NaiSecret, kind: &str) -> Vec<impl Future<Output = NaiResult>> {
        let mut tasks = Vec::new();
        if self.tags.is_empty() {
            match kind {
                "ss" => {
                    let pair = BuiltinPrompt::se_se();
                    self.tags = pair.1;
                    if self.name.is_empty() {
                        self.name = pair.0
                    }
                }
                _ => {
                    let pair = BuiltinPrompt::normal();
                    self.tags = pair.1;
                    if self.name.is_empty() {
                        self.name = pair.0
                    }
                }
            }
        }
        // let mut hasher = DefaultHasher::new();
        // self.tags.hash(&mut hasher);
        // self.name = hasher.finish().to_string();
        for _ in 1..=self.number {
            let mut rng = thread_rng();
            let seed = rng.gen();
            let builder =
                TaskBuilder { tags: self.tags.clone(), seed, dir: PathBuf::from(format!("target/nai/{}/", &self.name)) };
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
    #[allow(unused_assignments)]
    pub async fn run(self, cfg: &NaiConfig) -> NaiResult {
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
        println!("任务数: {:?}", tasks.iter().count());
        println!("协程数: {:?}", threads);
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
