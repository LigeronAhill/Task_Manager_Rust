use std::io::Write;
use crate::task_manager::TasksManager;
use crate::tasks::Task;

pub(crate) struct ConsoleManager {
    tasks_manager: TasksManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn print_menu(&self) {
        for (index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, menu_option);
        }
    }
    pub(crate) fn input(query: &str) -> std::io::Result<String> {
        print!("{}: ", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    pub(crate) fn process_command(&mut self) {
        match Self::input("Enter command index") {
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
                        match self.tasks_manager.find_task(name.as_str()){
                            None => println!("Task \"{}\" does not exist", name),
                            Some(index) => {
                                println!("Task \"{}\" found", name);
                                self.tasks_manager.tasks.get(index).unwrap().print_task()
                            }
                        }
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
                        let filename = match Self::input("Enter filename: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_manager.save_tasks_to_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg)
                        }
                    }
                    "7" => {
                        let filename = match Self::input("Enter filename: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_manager.load_tasks_from_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg)
                        }
                    }
                    _ => println!("Invalid command")
                }
            }
            Err(err) => println!("Error getting user input: {err}"),
        }
    }
}
