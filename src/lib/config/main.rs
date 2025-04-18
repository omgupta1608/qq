use chrono::Local;

use confy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn load_current_config(app_name: &str) -> Result<Config, confy::ConfyError> {
    let mut cfg: Config = confy::load(app_name, None)?;
    cfg.current_date = Local::now().format("%Y-%m-%d").to_string();
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

impl Config {
    pub fn get_items(&self) -> Option<&Vec<Item>> {
        self.data.get(&self.current_date)
    }

    pub fn print_items(&self) {
        match self.get_items() {
            Some(items) => {
                for (i, item) in items.clone().iter().enumerate() {
                    println!("{}. {}", i + 1, item.content)
                }
            }
            None => {
                println!("no items found for today")
            }
        }
    }
    pub fn add_item(&mut self, item_content: String) {
        let new_item = Item {
            done: false,
            content: item_content,
            created_at: Local::now().timestamp(),
        };
        self.data
            .entry(self.current_date.clone())
            .or_default()
            .push(new_item);
    }
}
