use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::console_manager::ConsoleManager;

#[derive(Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub(crate) name: String,
    description: String,
    priority: Priority,
    add_time: DateTime<Local>
}

impl Task {
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self {
            name,
            description,
            priority,
            add_time: Local::now()
        }
    }

    pub(crate) fn new_from_console() -> Self {
        let name = match ConsoleManager::input("Enter new task name") {
            Ok(name) => name,
            Err(err) => {
                println!("Error getting user input: {}, your task's name will be 'task'", err);
                "task".to_owned()
            }
        };
        let description = match ConsoleManager::input("Enter new task description") {
            Ok(description) => description,
            Err(err) => {
                println!("Error getting user input: {}, your task's description will be 'description'", err);
                "description".to_owned()
            }
        };
        let priority = match ConsoleManager::input("Enter new task priority") {
            Ok(priority_string) => match priority_string.to_lowercase().as_str() {
                "medium" => Priority::Medium,
                "high" => Priority::High,
                _ => Priority::Low,
            }
            Err(err) => {
                println!("Error getting user input: {}", err);
                Priority::Low
            }
        };
        Self::new(name, description, priority)
    }
    pub(crate) fn print_task(&self) {
        println!("Name: {} | Priority: {} | Added: {}\nDescription: {}\n",
                 self.name,
                 self.priority.to_string(),
                 self.add_time.format("%d.%m.%Y at %H:%M:%S"),
                 self.description,
        );
    }
}
