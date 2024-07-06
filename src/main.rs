use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Cli {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Cli::from_args();

    // ホームディレクトリを取得
    let home_dir = std::env::var("HOME")?;
    let config_path = PathBuf::from(home_dir).join(".aws/config");

    // コンフィグファイルのパスを表示
    println!("Config file path: {:?}", config_path);

    Ok(())
}
