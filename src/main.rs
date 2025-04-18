use clap::{Parser, Subcommand};
use confy;
use inquire::Text;
use qq::config::{self};
use std::error::Error;
use std::fs;

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
    #[command(about = "Get qq's config file path")]
    About,

    #[command(about = "Add a new todo item")]
    Add { item: String },

    #[command(about = "Mark an item as done")]
    Done {
        #[arg(long, short, action)]
        spill_over: bool,
        item_index: usize,
    },

    #[command(about = "Reset local data and config")]
    Reset,
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
            Command::Done {
                item_index,
                spill_over,
            } => {
                if spill_over {
                    current_cfg.mark_spillover_as_done(item_index)?;
                } else {
                    current_cfg.mark_as_done(item_index)?;
                }
                current_cfg.print_items();
            }
            Command::Reset => {
                let decision = Text::new("Are you sure? (y/n)").prompt();
                match decision {
                    Ok(d) => {
                        if d != "y" && d != "n" {
                            println!("invalid choice")
                        } else if d == "y" {
                            println!("deleting...");
                            let file = confy::get_configuration_file_path(APP_NAME, None)?;
                            fs::remove_file(file)?;
                            println!("done. run 'qq' to reinitialize config");
                        }
                    }
                    Err(_) => println!("something went wrong. please try again"),
                }
            }
        },
    }

    Ok(())
}
