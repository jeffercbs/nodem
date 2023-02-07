use crate::cli::args;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct App {
    #[clap(subcommand)]
    pub arguments: args::Actions,
}
