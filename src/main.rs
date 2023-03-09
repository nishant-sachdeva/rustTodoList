use core::panic;
use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>,
    // the idea is
    // for an item : key, the value will be True if it's active
    // and False if it's completed
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        // opening the database file
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("db.json")
            .expect("Failed to open file");

        // reading the file
        match serde_json::from_reader(file) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new()
            }),
            Err(e) => panic!("An error occured during opening the file: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("db.json")
            .expect("Failed to open file");
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(value) => Some(*value = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Invalid action");
    let item = std::env::args().nth(2).expect("Invalid item");

    println!("Action: {}", action);
    println!("Item: {}", item);

    let mut todo = Todo::new().expect("Failed to Initialize Todo List");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Action Completed. Todo Saved"),
            Err(e) => println!("Error: {}", e),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("{item} not found"),
            Some(_) => match todo.save() {
                Ok(_) => println!("Action Completed. Todo Saved"),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
