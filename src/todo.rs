use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs::create_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

pub trait Storage {
    fn load(&mut self);
    fn save(&self);
}

#[derive(Deserialize, Serialize)]
pub struct Todolist {
    pub items: Vec<Item>,
}

impl Storage for Todolist {
    fn load(&mut self) {
        match get_list_file_path() {
            Ok(path) => {
                if path.exists() {
                    match OpenOptions::new().read(true).open(path) {
                        Ok(file) => {
                            let metadata = file.metadata().expect("could not get file metadata");
                            if metadata.len() != 0 {
                                let reader = BufReader::new(file);
                                self.items =
                                    serde_json::from_reader::<BufReader<File>, Vec<Item>>(reader)
                                        .expect("Badly formated json");
                            }
                        }
                        Err(e) => {
                            println!("Error loading todolist file, did you command list/remove before ever adding an item? {}", e);
                        }
                    };
                }
            }

            Err(e) => {
                println!(
                    "Error could not access todo list file path, or create data directory {}",
                    e
                )
            }
        }; 
    }

    fn save(&self) {
        match get_list_file_path() {
            Ok(path) => {
                match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                {
                    Ok(mut file) => {
                        let serizlized_todo_list =
                            serde_json::to_string(&self.items).expect("invalid json");
                        file.write_all(serizlized_todo_list.as_bytes())
                            .expect("Failed to save todo list to file");
                    }
                    Err(e) => {
                        println!("Error opening todo list file for save operation: {}", e);
                    }
                };
            }

            Err(e) => {
                println!(
                    "Error could not access todo list file path, or create data directory: {}",
                    e
                )
            }
        };
    }
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub description: String,
    pub completed: bool,
}

impl fmt::Display for Todolist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (position, element) in self.items.iter().enumerate() {
            writeln!(f, "{}. {}", position + 1, element)?
        }

        Ok(())
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.completed {
            write!(f, "\u{2713} {}", self.description)
        } else {
            write!(f, "\u{0058} {}", self.description)
        }
    }
}

pub fn complete_item(itemid: u32) {
    let mut todolist = Todolist { items: Vec::new() };
    todolist.load();
    if todolist.items.is_empty() {
        println!("Cannot edit item id {} when list is empty", itemid)
    } else if itemid > todolist.items.len() as u32 {
        println!("Cannot edit item id {} because it does not exist", itemid)
    } else {
        todolist.items[itemid as usize - 1].completed = true;
        todolist.save();
    }
}

pub fn add_item(item: String) {
    let mut todolist = Todolist { items: Vec::new() };
    todolist.load();
    let new_item = Item {
        description: item,
        completed: false,
    };
    todolist.items.push(new_item);
    todolist.save();
}

pub fn remove_item(itemid: u32) {
    let mut todolist = Todolist { items: Vec::new() };
    todolist.load();
    let index = itemid - 1;
    if todolist.items.is_empty() {
        println!("Cannot remove item id {} from empty todo list", itemid)
    } else if itemid > todolist.items.len() as u32 {
        println!(
            "Cannot remove item id {} cannot remove an item that does not exist",
            itemid
        )
    } else {
        todolist.items.remove(index as usize);
        todolist.save();
    }
}

pub fn clear_items() {
    let todolist = Todolist { items: Vec::new() };
    todolist.save();
}

pub fn list_items() {
    let mut todolist = Todolist { items: Vec::new() };
    todolist.load();
    println!("{}", todolist);
}

fn get_list_file_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("data");

    if !dir.as_path().exists() {
        create_dir(dir.as_path())?;
    }

    dir.push("todo.json");
    Ok(dir)
}
