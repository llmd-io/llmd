use clap::Parser;
use tonic::{transport::Server, Request, Response, Status};
use crate::service::model_service::model_service_server::ModelServiceServer;

mod model;
mod service;
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // 初始化日志系统
    
    // 解析命令行参数
    let args = Args::parse();
    
    // 加载配置文件
    let config = config::Config::load(&args.config)?;
    let addr = config.server_addr().parse()?;
    let svc = service::ModelService::default();

    println!("Using config file: {}", args.config);
    println!("LLMD Server listening on {}", addr);

    Server::builder()
        .add_service(ModelServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
} 