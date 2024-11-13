mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    Cli::parse();
}
