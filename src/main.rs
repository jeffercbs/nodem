use clap::Parser;
use cli::args;
use console::style;
use http::node;

mod cli;
mod http;
mod nodem;

const ASCII_NODEM: &str = " 
    ▄▄    ▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄  ▄▄▄▄▄▄▄ ▄▄   ▄▄ 
    █  █  █ █       █      ██       █  █▄█  █
    █   █▄█ █   ▄   █  ▄    █    ▄▄▄█       █
    █       █  █ █  █ █ █   █   █▄▄▄█       █
    █  ▄    █  █▄█  █ █▄█   █    ▄▄▄█       █
    █ █ █   █       █       █   █▄▄▄█ ██▄██ █
    █▄█  █▄▄█▄▄▄▄▄▄▄█▄▄▄▄▄▄██▄▄▄▄▄▄▄█▄█   █▄█
";

fn os_info() {
    let info = nodem::os::get_arch();

    println!("Operating System -> {}", style(info.os).cyan());
    println!("Architecture -> {}", style(info.arch).blue());
    println!("Path -> {}", style(info.path).yellow().bold());
}

#[tokio::main]
async fn main() {
    let app = cli::new::App::parse();

    println!("{}", ASCII_NODEM);
    match app.arguments {
        args::Actions::Info => os_info(),
        args::Actions::Install(args) => node::download(args.version).await,
        args::Actions::Uninstall(args) => println!("{:?}", args),
        args::Actions::Use(args) => println!("{:?}", args),
        args::Actions::List(args) => node::available(args.remote).await,
    }
}
