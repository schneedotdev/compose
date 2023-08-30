use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ComposeArgs {
  /// the name of the struct to be generated
  pub struct_name: String,
}