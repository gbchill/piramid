use std::fs;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Priority { // pub added
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem { // pub added
    pub id: u64, // pub added to fields
    pub name: String,
    pub completed: bool,
    pub priority: Priority,
    pub deadline: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList { // pub added
    pub items: Vec<TodoItem>,
    pub next_id: u64,
}

impl TodoList {
    pub fn new() -> TodoList { // pub added
        match fs::read_to_string("db.json") {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(list) => list,
                    Err(_) => TodoList { items: Vec::new(), next_id: 1 }
                }
            },
            Err(_) => TodoList { items: Vec::new(), next_id: 1 }
        }
    }

    pub fn add_item(&mut self, name: String, priority: Option<String>, deadline: Option<String>) -> bool {
        if self.items.iter().any(|item| item.name.to_lowercase() == name.to_lowercase()) {
            println!("Task with the same name already exists");
            return false;
        }

        let id = self.next_id;
        let priority_enum = match priority {
            Some(p) => match p.to_lowercase().as_str() {
                "low" => Priority::Low,
                "medium" => Priority::Medium,
                "high" => Priority::High,
                _ => Priority::Medium,
            },
            None => Priority::Medium,
        };

        let deadline_date = match deadline {
            Some(d) => match NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                Ok(date) => Some(date),
                Err(_) => {
                    println!("Invalid date format. Use YYYY-MM-DD");
                    None
                }
            },
            None => None,
        };

        let new_item = TodoItem {
            id,
            name,
            completed: false,
            priority: priority_enum,
            deadline: deadline_date,
        };

        self.next_id += 1;
        self.items.push(new_item);
        println!("Task added successfully");
        true
    }

    pub fn edit_task_name(&mut self, id: u64, new_name: String) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            if item.name.to_lowercase() == new_name.to_lowercase() {
                return false; 
            }
            item.name = new_name;
            println!("Name updated");
            return true;
        }
        false
    }

    pub fn edit_task_priority(&mut self, id: u64, new_priority: Priority) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.priority = new_priority;
            println!("Priority updated");
            return true;
        }
        false
    }

    pub fn edit_task_deadline(&mut self, id: u64, new_deadline: Option<String>) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
             item.deadline = match new_deadline {
                Some(d) => match NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                    Ok(date) => Some(date),
                    Err(_) => {
                        println!("Invalid date format.");
                        return false;
                    }
                },
                None => None,
            };
            println!("Deadline updated");
            return true;
        }
        false
    }

    pub fn complete_item(&mut self, id: u64) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.completed = true;
            println!("Task marked as completed");
            return true;
        }
        false
    }

    pub fn delete_item(&mut self, id: u64) -> bool {
        let indexes = self.items.iter().position(|item| item.id == id);
        match indexes {
            Some(index) => {
                self.items.remove(index);
                true
            },
            None => false
        }
    }

    pub fn delete_all_completed(&mut self) {
        self.items.retain(|item| !item.completed);
        println!("All completed tasks deleted");
    }

    pub fn sort_by_priority(&mut self) {
        self.items.sort_by(|a, b| b.priority.cmp(&a.priority));
        println!("Tasks sorted by priority");
    }

    pub fn sort_by_deadline(&mut self) {
        self.items.sort_by(|a, b| a.deadline.cmp(&b.deadline));
        println!("Tasks sorted by deadline");
    }

    // Optimization: Instead of returning Vec<&TodoItem>, we just print here
    // to avoid lifetime complexity in main.rs for now.
    pub fn search(&self, query: &str) {
        println!("--- Search Results for '{}' ---", query);
        let mut found = false;
        for item in &self.items {
            if item.name.to_lowercase().contains(&query.to_lowercase()) {
                self.print_item(item);
                found = true;
            }
        }
        if !found { println!("No tasks found."); }
        println!("-------------------------------");
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let content = serde_json::to_string_pretty(&self)?;
        fs::write("db.json", content)?;
        Ok(())
    }

    // Helper to print a single item (DRY principle)
    fn print_item(&self, item: &TodoItem) {
        let status = if item.completed { "[X]" } else { "[ ]" };
        let deadline_str = match item.deadline {
            Some(date) => date.to_string(),
            None => "No deadline".to_string(),
        };
        println!("{}\t{}\t{:?}\t\t{}\t{}", item.id, status, item.priority, deadline_str, item.name);
    }

    pub fn print(&self) {
        println!("================================================================");
        println!("ID\tStatus\tPriority\tDue Date\tName");
        println!("================================================================");
        for item in &self.items {
            self.print_item(item);
        }
        println!("================================================================");
    }
}
