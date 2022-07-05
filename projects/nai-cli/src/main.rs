use std::{fs::read_to_string, io::stdin};

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use toml::from_str;

use novel_ai::{NaiError, NaiResult, NaiSecret};

pub use self::builtin::BuiltinPrompt;

mod builtin;
mod command_args;
mod task_builder;
mod with_tags;

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
    /// 提示词, 逗号隔开
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

impl Default for CommandArgs {
    fn default() -> Self {
        Self { name: "".to_string(), tags: "".to_string(), threads: 3, number: 5, frame: 1, step: 16 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NaiConfig {
    nai: NaiSecret,
}

impl NaiConfig {
    pub fn load() -> NaiResult<Self> {
        let toml = match read_to_string("nai.toml") {
            Ok(o) => o,
            Err(_) => return Err(NaiError::ParseError("配置文件 nai.toml 读取失败".to_string())),
        };
        match from_str(&toml) {
            Ok(o) => Ok(o),
            Err(e) => return Err(NaiError::ParseError(e.to_string())),
        }
    }
}

impl Default for NaiApp {
    fn default() -> Self {
        Self { command: Commands::SS(CommandArgs::default()) }
    }
}

#[tokio::main]
async fn main() -> NaiResult {
    let args = NaiApp::try_parse().unwrap_or_default();
    let config = NaiConfig::load()?;
    args.command.run(&config).await?;
    press_btn_continue::wait("按任意键退出...").unwrap();
    Ok(())
}
