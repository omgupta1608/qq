use chrono::Local;
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

    pub fn get_status_items(&self, done: bool) -> Option<Vec<Item>> {
        let mut status_items: Vec<Item> = vec![];
        match self.get_all_items() {
            Some(items) => {
                for item in items.clone() {
                    if item.done == done {
                        status_items.push(item.clone());
                    }
                }
                Some(status_items)
            }
            None => {
                println!("no items found for today");
                None
            }
        }
    }

    pub fn print_items(&self) {
        println!("Today is: {}\n\n", self.current_date);
        match self.get_all_items() {
            Some(items) => {
                for (i, item) in items.clone().iter().enumerate() {
                    if item.done {
                        println!("[x] {}. {}", i + 1, item.content)
                    } else {
                        println!("[ ] {}. {}", i + 1, item.content)
                    }
                }
            }
            None => {
                println!("no items found for today")
            }
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

    pub fn mark_as_done(&mut self, item_index: usize) -> Result<(), Box<dyn Error>> {
        match self.get_all_items_mut() {
            Some(items) => {
                if item_index > items.len() {
                    println!("invalid item index");
                } else {
                    items[item_index - 1].mark_as_done();
                    confy::store(APP_NAME, None, &self)?;
                }
            }
            None => {
                println!("no items found for today")
            }
        }
        Ok(())
    }
}
