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



pub mod builtin;
pub mod with_tags;
pub mod command_args;
pub mod task_builder;

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
    threads: usize,
    /// 生成多少组图片, 默认 5 组
    #[arg(short, long, default_value_t = 5)]
    number: u32,
    /// 每组图片生成几张, 默认 1 张
    #[arg(short, long, default_value_t = 1)]
    frame: u32,
    /// 每组中图片的变化量, 默认 16
    #[arg(short, long, default_value_t = 16)]
    step: u32,
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

#[tokio::main]
async fn main() -> NaiResult {
    let args = NaiApp::parse();
    let config = NaiConfig::load()?;
    args.command.run(&config).await?;
    Ok(())
}

