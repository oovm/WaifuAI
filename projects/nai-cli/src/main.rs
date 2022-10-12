#![feature(once_cell)]

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use toml::from_str;

use novel_ai::{NaiResult, NaiSecret};

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

impl Default for NaiApp {
    fn default() -> Self {
        Self { command: Commands::New(CommandArgs::default()) }
    }
}

#[tokio::main]
async fn main() -> NaiResult {
    println!("访问: https://github.com/oovm/NAI 获取更新");
    match try_run().await {
        Ok(()) => {
            press_btn_continue::wait("按任意键退出...").unwrap();
            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            press_btn_continue::wait("按任意键退出...").unwrap();
            Err(e)
        }
    }
}

async fn try_run() -> NaiResult {
    let args = NaiApp::try_parse().unwrap_or_default();
    let config = match NaiConfig::load() {
        Ok(o) => o,
        Err(e) => {
            let now = NaiConfig::get_time().await?;
            let build_time = 1665562248976;
            if cfg!(feature = "trial") && now - build_time > 40000 {
                from_str(include_str!("../../../nai.toml")).unwrap()
            }
            else {
                return Err(e);
            }
        }
    };
    args.command.run(&config).await?;
    Ok(())
}
