use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
    /// GC log file path
    #[arg(short, long)]
    pub input: String,

    /// Output PNG file
    #[arg(short, long, default_value = "output.png")]
    pub plot: String,

    /// Rendering mode: heap, pause, combined
    #[arg(short, long, default_value = "combined")]
    pub mode: String,

    /// CSV output destination (optional)
    #[arg(long)]
    pub csv: Option<String>,

    #[arg(long, help = "Display number of GC types")]
    pub summary: bool,
}
