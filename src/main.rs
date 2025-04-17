use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "qq")]
#[command(about = "qq", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    About,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::About => {
            println!("qq cli")
        }
    }
}
