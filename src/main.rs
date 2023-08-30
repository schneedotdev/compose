mod args;

use args::{ComposeArgs, Entity};
use clap::Parser;

fn main() {
    let args = ComposeArgs::parse();

    let props = match args.entity {
        Entity::Struct(props) => props
    };

    println!("{:?}", props.name);
}