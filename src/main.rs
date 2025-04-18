use std::error::Error;

use clap::{Parser, Subcommand};
use confy;
use qq::config;

const APP_NAME: &str = "qq";

#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(about = APP_NAME, version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    About,
    Add { item: String },
    Remove,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // load current config
    let mut current_cfg = config::main::load_current_config(APP_NAME)?;

    match cli.command {
        // default - lists items
        None => {
            current_cfg.print_items();
        }
        // executes subcommands
        Some(command) => match command {
            Command::About => {
                let file = confy::get_configuration_file_path(APP_NAME, None)?;
                println!("The qq config file path is: {:#?}", file);
            }
            Command::Add { item } => {
                current_cfg.add_item(item);
                confy::store(APP_NAME, None, current_cfg)?;
            }
            Command::Remove => {
                println!("{APP_NAME} remove")
            }
        },
    }
    Ok(())
}
