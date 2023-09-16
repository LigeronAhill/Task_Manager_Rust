use crate::console_manager::ConsoleManager;

mod tasks;
mod task_manager;
mod console_manager;

fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();

    loop {
        manager.process_command();
        println!();
    }
}
