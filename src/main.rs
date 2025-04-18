use clap::{Parser, Subcommand};
use confy;
use qq::config;
use std::error::Error;

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
    Done { item_index: usize },
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
                current_cfg.add_item(item)?;
                current_cfg.print_items();
            }
            Command::Done { item_index } => {
                current_cfg.mark_as_done(item_index)?;
                current_cfg.print_items();
            }
        },
    }

    Ok(())
}
