mod args;
mod gen;

use clap::Parser;
use args::ComposeArgs;
use gen::{create_template, generate_code};

fn main() {
    let args = ComposeArgs::parse();
    let (name, template) = create_template(args.entity);
    generate_code(name, template);
}