use std::{env::current_exe, fs::read_to_string, future::Future, str::FromStr};

use futures_util::StreamExt;
use rand::{thread_rng, Rng};
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use toml::from_str;

use novel_ai::{NaiError, NaiResult, NaiSecret};

use crate::{builtin::BUILTIN_PROMPTS, task_builder::TaskBuilder, CommandArgs, Commands, NaiConfig};

impl CommandArgs {
    pub fn prepare_tasks(mut self, secret: NaiSecret, kind: &str) -> Vec<impl Future<Output = NaiResult>> {
        let mut tasks = Vec::new();
        if self.tags.is_empty() {
            match kind {
                "ss" => {
                    let pair = BUILTIN_PROMPTS.se_se();
                    self.tags = pair.1;
                    if self.name.is_empty() {
                        self.name = pair.0
                    }
                }
                _ => {
                    let pair = BUILTIN_PROMPTS.normal();
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
            let builder = match TaskBuilder::new(&self.tags, &self.name, seed) {
                Ok(o) => o,
                Err(e) => {
                    println!("TaskBuilder::new {}", e);
                    return tasks;
                }
            };
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

impl NaiConfig {
    pub fn load() -> NaiResult<Self> {
        let config_path = current_exe()?.with_file_name("nai.toml");
        let toml = match read_to_string(config_path) {
            Ok(o) => o,
            Err(_) => {
                return Err(NaiError::ParseError("配置文件 nai.toml 读取失败".to_string()));
            }
        };
        match from_str(&toml) {
            Ok(o) => Ok(o),
            Err(e) => return Err(NaiError::ParseError(e.to_string())),
        }
    }
}

impl NaiConfig {
    pub async fn get_time() -> NaiResult<u64> {
        let tao_bao: TaoBao = Client::default()
            .request(Method::GET, "https://api.m.taobao.com/rest/api3.do?api=mtop.common.getTimestamp")
            .send()
            .await?
            .json()
            .await?;
        Ok(u64::from_str(&tao_bao.data.t)?)
    }
}

#[derive(Serialize, Deserialize)]
struct TaoBaoTime {
    pub t: String,
}

#[derive(Serialize, Deserialize)]
struct TaoBao {
    pub api: String,
    pub v: String,
    pub ret: Vec<String>,
    pub data: TaoBaoTime,
}
