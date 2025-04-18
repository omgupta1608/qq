use std::error::Error;

use clap::{Parser, Subcommand};
use confy;
use serde::{Deserialize, Serialize};
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
    Add { item: i32 },
    Remove,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Config {
    version: String,
    items: Vec<i32>,
}

impl Config {
    fn add(&mut self, item: i32) {
        self.items.push(item);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut current_cfg: Config = confy::load(APP_NAME, None)?;
    match cli.command {
        // default - lists items
        None => {
            println!("{:?}", current_cfg.items)
        }
        // executes subcommands
        Some(command) => match command {
            Command::About => {
                let file = confy::get_configuration_file_path(APP_NAME, None)?;
                println!("The qq config file path is: {:#?}", file);
            }
            Command::Add { item } => {
                current_cfg.add(item);
                confy::store(APP_NAME, None, current_cfg)?;
            }
            Command::Remove => {
                println!("{APP_NAME} remove")
            }
        },
    }
    Ok(())
}
