use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ComposeArgs {
    #[clap(subcommand)]
    pub entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Entity {
    /// generate a struct.
    Struct(StructProperties),
}

#[derive(Debug, Args)]
pub struct StructProperties {
    pub name: String,
}
