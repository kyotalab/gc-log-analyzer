use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
    /// GCログファイルパス
    #[arg(short, long)]
    pub input: String,

    /// 出力PNGファイル
    #[arg(short, long, default_value = "output.png")]
    pub plot: String,

    /// 描画モード: heap, pause, combined
    #[arg(short, long, default_value = "combined")]
    pub mode: String,

    /// CSV出力先（省略可）
    #[arg(long)]
    pub csv: Option<String>,

    #[arg(long, help = "Display number of GC types")]
    pub summary: bool,
}
