pub use clap::Parser;
pub mod grid_util;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub input: String,
}
