use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tasks::Task;
use crate::tasks;

pub struct TasksManager {
    pub(crate) tasks: Vec<Task>
}

impl TasksManager {
    pub(crate) fn new() -> Self {
        Self {
            tasks: Vec::new()
        }
    }

    pub(crate) fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    pub(crate) fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub(crate) fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("Task \"{}\" removed successfully", name))
        } else {
            Err(format!("Task \"{}\" does not exist", name))
        }
    }

    pub(crate) fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|task| task.name == name)
    }

    pub(crate) fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String> {
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

    pub(crate) fn save_tasks_to_file(&self, filename: &str) -> Result<String, String> {
        if !Path::new((filename.to_string() + ".json").as_str()).exists() {
            let file = match File::create((filename.to_string() + ".json").as_str()) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {}", err)),
            };
            match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("Data saved".to_owned()),
                Err(err) => Err(format!("Error saving data: {}", err))
            }

        } else {
            Err("File already exists".to_owned())
        }
    }

    pub(crate) fn load_tasks_from_file(&mut self, filename: &str) -> Result<String, String> {
        if Path::new((filename.to_string() + ".json").as_str()).exists() {
            let file = match File::open((filename.to_string() + ".json").as_str()) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error opening file: {}", err)),
            };
            let reader = BufReader::new(file);
            self.tasks = match serde_json::from_reader(reader) {
                Ok(tasks) => tasks,
                Err(err) => return Err(format!("Error loading data: {}", err))
            };
            Ok("Data loaded".to_owned())
        } else {
            Err("File \"{filename}\" does not exist".to_owned())
        }
    }
}
