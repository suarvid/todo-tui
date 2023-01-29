use self::todoitem::TodoItem;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io;

pub mod todoitem;

const DEFAULT_SAVE_PATH: &str = "/.todo_tui/items.json";

pub struct Backend {
    todo_items: Vec<TodoItem>,
}

impl Backend {
    pub fn new() -> Backend {
        Backend {
            todo_items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, title: &str) {
        self.todo_items.push(TodoItem::new_item(title));
    }

    pub fn remove_item_at_index(&mut self, index: usize) {
        self.todo_items.remove(index);
    }

    pub fn save_items(&self) {
        let home_path = env::var("HOME");
        if home_path.is_err() {
            print!("Failed to get value of $HOME environment variable"); // Maybe try writing to some other hard-coded path?
        } else {
            let home_path = home_path.unwrap();
            let save_path = home_path + DEFAULT_SAVE_PATH;
            match serde_json::to_string(&self.todo_items) {
                Ok(serialized_string) => match fs::write(save_path, serialized_string) {
                    Ok(_) => println!("Successfully saved items."),
                    Err(err) => {
                        println!("Failed to save items: {}", err);
                    }
                },
                Err(err) => {
                    println!("Failed to serialize items: {}", err);
                }
            }
        }
    }

    pub fn restore_items(&mut self) -> Result<(), std::io::Error> {
        if let Ok(home_path) = env::var("HOME") {
            let save_path = home_path + DEFAULT_SAVE_PATH;
            let mut handle = File::open(save_path)?;
            let mut buf = String::new();
            handle.read_to_string(&mut buf)?;
            self.todo_items = serde_json::from_str(buf.as_str())?;
            return Ok(());
        }

        Err(io::Error::new(io::ErrorKind::Other, "Failed to read value of $HOME environment variable."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_first_item() {
        let mut bnd = Backend::new();
        bnd.add_item("First");
        bnd.add_item("Second");
        bnd.add_item("Third");
        assert_eq!(bnd.todo_items.len(), 3);
        bnd.remove_item_at_index(0);
        assert_eq!(bnd.todo_items.len(), 2);
        assert_eq!(bnd.todo_items.get(0).unwrap().get_title(), "Second");
        assert_eq!(bnd.todo_items.get(1).unwrap().get_title(), "Third");
    }

    #[test]
    fn test_remove_last_item() {
        let mut bnd = Backend::new();
        bnd.add_item("First");
        bnd.add_item("Second");
        bnd.add_item("Third");
        assert_eq!(bnd.todo_items.len(), 3);
        bnd.remove_item_at_index(2);
        assert_eq!(bnd.todo_items.len(), 2);
        assert_eq!(bnd.todo_items.get(0).unwrap().get_title(), "First");
        assert_eq!(bnd.todo_items.get(1).unwrap().get_title(), "Second");
    }

    // TODO: Change this test so it doesn't actually write to disk
    #[test]
    fn test_save_items() {
        let mut bnd = Backend::new();
        bnd.add_item("First Test Item");
        bnd.add_item("Second Test Item");
        bnd.save_items();
    }

    // TODO: make test for restoring items

    #[test]
    fn test_get_home_path() {
        assert!(env::var("HOME").is_ok());
    }
}
