use chrono::Local;
use colored::Colorize;
use confy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

const APP_NAME: &str = "qq";
pub fn load_current_config(app_name: &str) -> Result<Config, confy::ConfyError> {
    let mut cfg: Config = confy::load(app_name, None)?;
    cfg.current_date = Local::now().format("%d %B %Y").to_string();
    Ok(cfg)
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    current_date: String,
    data: HashMap<String, Vec<Item>>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Item {
    done: bool,
    content: String,
    created_at: i64,
}

impl Item {
    pub fn mark_as_done(&mut self) {
        self.done = true;
    }
}

impl Config {
    pub fn get_all_items(&self) -> Option<&Vec<Item>> {
        self.data.get(&self.current_date)
    }

    pub fn get_all_items_mut(&mut self) -> Option<&mut Vec<Item>> {
        self.data.get_mut(&self.current_date)
    }

    pub fn get_all_spillover_refs(&self) -> Vec<(String, usize, &Item)> {
        let mut result = vec![];

        let mut dates: Vec<&String> = self
            .data
            .keys()
            .filter(|d| *d != &self.current_date)
            .collect();

        dates.sort();

        for date in dates {
            if let Some(items) = self.data.get(date) {
                for (i, item) in items.iter().enumerate() {
                    if !item.done {
                        result.push((date.clone(), i, item));
                    }
                }
            }
        }

        result
    }

    pub fn print_items(&self) {
        println!("Today is: {}\n", self.current_date);
        match self.get_all_items() {
            Some(items) => {
                for (i, item) in items.iter().enumerate() {
                    if item.done {
                        println!("{}. {}", i + 1, item.content.green())
                    } else {
                        println!("{}. {}", i + 1, item.content)
                    }
                }
            }
            None => {
                println!("no items found for today")
            }
        }
        println!("\n\n--- SPILL OVERS\n");
        self.print_spillover_items();
    }

    pub fn print_spillover_items(&self) {
        let spillover_items = self.get_all_spillover_refs();

        if spillover_items.is_empty() {
            println!("No spillover tasks 🎉");
            return;
        }

        for (i, (date, _idx, item)) in spillover_items.iter().enumerate() {
            println!("{}. {} ({})", i + 1, item.content, date);
        }
    }

    pub fn add_item(&mut self, item_content: String) -> Result<(), Box<dyn Error>> {
        let new_item = Item {
            done: false,
            content: item_content,
            created_at: Local::now().timestamp(),
        };
        self.data
            .entry(self.current_date.clone())
            .or_default()
            .push(new_item);

        confy::store(APP_NAME, None, &self)?;
        Ok(())
    }

    pub fn mark_spillover_as_done(&mut self, item_index: usize) -> Result<(), Box<dyn Error>> {
        if item_index == 0 {
            return Err("Invalid index".into());
        }

        let mut dates: Vec<String> = self
            .data
            .keys()
            .filter(|d| *d != &self.current_date)
            .cloned()
            .collect();

        dates.sort();

        let mut flat_index = 0;

        for date in dates {
            if let Some(items) = self.data.get_mut(date.as_str()) {
                for i in 0..items.len() {
                    if !items[i].done {
                        flat_index += 1;

                        if flat_index == item_index {
                            items[i].mark_as_done();
                            confy::store(APP_NAME, None, &self)?;
                            return Ok(());
                        }
                    }
                }
            }
        }

        return Err("Invalid index".into());
    }

    pub fn mark_as_done(&mut self, item_index: usize) -> Result<(), Box<dyn Error>> {
        match self.get_all_items_mut() {
            Some(items) => {
                if item_index == 0 || item_index > items.len() {
                    return Err("Invalid index".into());
                } else {
                    items[item_index - 1].mark_as_done();
                    confy::store(APP_NAME, None, &self)?;
                }
            }
            None => println!("no items found for today"),
        }
        Ok(())
    }
}
