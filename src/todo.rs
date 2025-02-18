use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs::create_dir;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Todolist {
    pub items: Vec<Item>,
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
    let mut todolist = load_todo_list();
    if todolist.items.is_empty() {
        println!("Cannot edit item id {} when list is empty", itemid)
    } else if itemid > todolist.items.len() as u32 {
        println!(
            "Cannot edit item id {} because it does not exist",
            itemid
        )
    } else {
        todolist.items[itemid as usize - 1].completed = true;
        save_todo_list(todolist);
    }
}

pub fn add_item(item: String) {
    let mut todolist = load_todo_list();
    let new_item = Item {description: item, completed: false};
    todolist.items.push(new_item);
    save_todo_list(todolist);
}

pub fn remove_item(itemid: u32) {
    let mut todolist = load_todo_list();
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
        save_todo_list(todolist);
    }
}

pub fn clear_items() {
    let todolist = Todolist{items: Vec::new()};
    save_todo_list(todolist);
}

pub fn list_items() {
    let todolist = load_todo_list();
    println!("{}", todolist);
}

pub fn load_todo_list() -> Todolist {
    let mut todolist = Todolist { items: Vec::new() };
    match get_list_file_path() {
        Ok(path) => match OpenOptions::new().read(true).open(path) {
            Ok(file) => {
                let metadata = file.metadata().expect("could not get file metadata");
                if metadata.len() != 0 {
                    let reader = BufReader::new(file);
                    todolist = serde_json::from_reader(reader).expect("Badly formated json");
                }
            }
            Err(e) => {
                println!("Error opening todo list file, did you list/remove before ever adding an item? {}", e);
            }
        },

        Err(e) => {
            println!(
                "Error occured when creating todo list file path from executable file path {}",
                e
            )
        }
    }

    todolist
}

pub fn save_todo_list(list: Todolist) {
    match get_list_file_path() {
        Ok(path) => match OpenOptions::new().write(true).create(true).truncate(true).open(path) {
            Ok(mut file) => {
                let serizlized_todo_list = serde_json::to_string(&list).expect("invalid json");
                file.write_all(serizlized_todo_list.as_bytes())
                    .expect("Failed to save todo list to file");
            }
            Err(e) => {
                println!("Error opening todo list file {}", e);
            }
        },

        Err(e) => {
            println!(
                "Error occured when creating todo list file path from executable file path {}",
                e
            )
        }
    }
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
