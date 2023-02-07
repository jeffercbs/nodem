use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[clap(
        about = "Display information of the machine on which Node is running",
        long_flag = "info"
    )]
    Info,
    #[clap(
        about = "List the installations. Type \"remote\" at the end to see what can be installed.",
        long_flag = "list",
        short_flag = 'l'
    )]
    List(ListArgs),
    #[clap(
        about = "Specify the version of Node you want to install.",
        long_flag = "install",
        short_flag = 'i'
    )]
    Install(NodeArg),
    #[clap(
        about = "Uninstalls the specified version of Node.js",
        long_flag = "uninstall",
        short_flag = 'u'
    )]
    Uninstall(NodeArg),
    #[clap(
        about = "Switch to use the specified version. Optionally use \"latest\".",
        long_flag = "use"
    )]
    Use(NodeArg),
}

#[derive(Debug, Args)]
pub struct NodeArg {
    pub version: String,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[clap(default_value = "")]
    pub remote: String,
}
