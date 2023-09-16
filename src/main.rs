use std::io::Write;
use chrono::{DateTime, Local};

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

struct Task {
    name: String,
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

    fn new_from_console() -> Self {
        let name = match ConsoleManager::input("Enter new task name: ") {
            Ok(name) => name,
            Err(err) => {
                println!("Error getting user input: {}, your task's name will be 'task'", err);
                "task".to_owned()
            }
        };
        let description = match ConsoleManager::input("Enter new task description: ") {
            Ok(description) => description,
            Err(err) => {
                println!("Error getting user input: {}, your task's description will be 'description'", err);
                "description".to_owned()
            }
        };
        let priority = match ConsoleManager::input("Enter new task priority: ") {
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
    fn print_task(&self) {
        println!("Name: {} | Priority: {} | Added: {}\nDescription: {}\n",
                 self.name,
                 self.priority.to_string(),
                 self.add_time.format("%d.%m.%Y at %H:%M:%S"),
                 self.description,
        );
    }
}

struct TasksManager {
    tasks: Vec<Task>
}

impl TasksManager {
    fn new() -> Self {
        Self {
            tasks: Vec::new()
        }
    }

    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("Task \"{}\" removed successfully", name))
        } else {
            Err(format!("Task \"{}\" does not exist", name))
        }
    }

    fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|task| task.name == name)
    }

    fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            match self.tasks.get_mut(index) {
                None => Err(format!("Task \"{}\" does not exist", name)),
                Some(task) => {
                    *task = updated_task;
                    Ok(format!("Task \"{}\" updated successfully", name))
                }
            }
        } else {
            Err(format!("Task \"{}\" does not exist", name))
        }

    }
}

struct ConsoleManager {
    tasks_manager: TasksManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    fn new() -> Self {
        Self {
            tasks_manager: TasksManager::new(),
            menu_options: vec![
                "Add task".to_owned(),
                "Find task".to_owned(),
                "Edit task".to_owned(),
                "Remove task".to_owned(),
                "Print tasks".to_owned(),
                "Store tasks to file".to_owned(),
                "Load tasks from file".to_owned(),
            ]
        }
    }

    fn print_menu(&self) {
        for (index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, menu_option);
        }
    }
    fn input(query: &str) -> std::io::Result<String> {
        print!("{}: ", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn process_command(&mut self) {
        match Self::input("Enter command index: ") {
            Ok(command) => {
                match command.as_str() {
                    "1" => {
                        self.tasks_manager.add_task(Task::new_from_console());
                    }

                    "2" => {
                        let name = match Self::input("Enter task name to find: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };
                        self.tasks_manager.find_task(name.as_str());
                    }
                    "3" => {
                        let name = match Self::input("Enter task name to edit: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_manager.edit_task(name.as_str(), Task::new_from_console()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg)
                        }
                    }

                    "4" => {
                        let name = match Self::input("Enter task name to remove: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_manager.remove_task(name.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg)
                        }
                    }

                    "5" => {
                        self.tasks_manager.print_tasks()
                    }

                    "6" => {
                        println!("Try again later")
                    }
                    "7" => {
                        println!("Try again later")
                    }
                    _ => println!("Invalid command")
                }
            }
            Err(err) => println!("Error getting user input: {err}"),
        }
    }
}

fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();

    loop {
        manager.process_command();
    }
}
