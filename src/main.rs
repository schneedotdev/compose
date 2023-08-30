mod args;

use args::ComposeArgs;
use clap::Parser;

fn main() {
    let args = ComposeArgs::parse();
    println!("{:?}", args);
}
