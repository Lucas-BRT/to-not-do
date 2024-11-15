use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use uuid::{self, Uuid};

use crate::file_management::{self, Task};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
#[command(rename_all = "kebab-case")]
pub enum Commands {
    #[clap(name = "add", about = "Add a new task")]
    Add { task_description: String },
    #[clap(name = "update", about = "Update an existing task")]
    Update {
        task_id: Uuid,
        task_description: String,
    },
    #[clap(name = "delete", about = "Delete a task")]
    Delete { task_id: Uuid },
    #[clap(name = "list", about = "List tasks")]
    List { filter: Option<TaskState> },
    #[clap(name = "mark-done", about = "Mark a task as done")]
    MarkDone { task_id: Uuid },
    #[clap(name = "mark-in-progress", about = "Mark a task as in progress")]
    MarkInProgress { task_id: Uuid },
}

#[derive(Debug, ValueEnum, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskState {
    Todo,
    InProgress,
    Done,
}

pub fn handle_commands(args: Args, db_manager: &mut file_management::DatabaseManager) {
    match args.command {
        Commands::Add { task_description } => {
            handle_add_task(task_description, db_manager);
        }
        Commands::Update {
            task_id,
            task_description,
        } => {
            handle_update_task(task_id, task_description, db_manager);
        }
        Commands::Delete { task_id } => {
            handle_delete_task(task_id, db_manager);
        }
        Commands::List { filter } => {
            handle_list_tasks(db_manager, filter);
        }
        Commands::MarkDone { task_id } => {
            handle_mark_done(task_id, db_manager);
        }
        Commands::MarkInProgress { task_id } => {
            handle_mark_in_progress(task_id, db_manager);
        }
    }
}

fn handle_add_task(task_description: String, db_manager: &mut file_management::DatabaseManager) {
    println!("Adding task: {}", task_description);

    let task = Task::new(&task_description);

    match db_manager.add_task(&task) {
        Ok(_) => println!("Task added successfully"),
        Err(_) => println!("Failed to add task"),
    }
}

fn handle_update_task(
    task_id: Uuid,
    task_description: String,
    db_manager: &mut file_management::DatabaseManager,
) {
    println!("Updating task: {}", task_description);

    match db_manager.update_description(task_id, &task_description) {
        Ok(_) => println!("Task updated successfully"),
        Err(_) => println!("Task not found"),
    };
}

fn handle_delete_task(task_id: Uuid, db_manager: &mut file_management::DatabaseManager) {
    match db_manager.delete_task(task_id) {
        Ok(_) => println!("Task deleted successfully"),
        Err(_) => println!("Task not found"),
    };
}

fn handle_list_tasks(db_manager: &mut file_management::DatabaseManager, filter: Option<TaskState>) {
    if let Some(filter) = filter {
        println!("Listing tasks with filter: {:?}", filter);
        let filtered_tasks = db_manager.filter_tasks(filter);

        if filtered_tasks.is_empty() {
            println!("No tasks found with the specified filter");
        } else {
            for task in filtered_tasks {
                println!("------------------");
                println!("{}", task);
            }
            println!("------------------");
        }
    } else {
        let tasks = match db_manager.get_tasks() {
            Ok(tasks) => tasks,
            Err(_) => {
                println!("Failed to retrieve tasks");
                return;
            }
        };

        if tasks.is_empty() {
            println!("No tasks found");
        } else {
            for task in tasks {
                println!("------------------");
                println!("{}", task);
            }
            println!("------------------");
        }
    }
}

fn handle_mark_done(task_id: Uuid, db_manager: &mut file_management::DatabaseManager) {
    match db_manager.set_task_state(task_id, TaskState::Done) {
        Ok(_) => println!("Task marked as done"),
        Err(_) => println!("Task not found"),
    };
}

fn handle_mark_in_progress(task_id: Uuid, db_manager: &mut file_management::DatabaseManager) {
    match db_manager.set_task_state(task_id, TaskState::InProgress) {
        Ok(_) => println!("Task marked as in progress"),
        Err(_) => println!("Task not found"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use uuid::Uuid;

    #[test]
    fn test_add_command() {
        let args = Args::parse_from(["to-not-do", "add", "Test task"]);
        if let Commands::Add { task_description } = args.command {
            assert_eq!(task_description, "Test task");
        } else {
            panic!("Expected Add command");
        }
    }

    #[test]
    fn test_update_command() {
        let task_id = Uuid::new_v4();
        let args = Args::parse_from(["to-not-do", "update", &task_id.to_string(), "Updated task"]);
        if let Commands::Update {
            task_id: id,
            task_description,
        } = args.command
        {
            assert_eq!(id, task_id);
            assert_eq!(task_description, "Updated task");
        } else {
            panic!("Expected Update command");
        }
    }

    #[test]
    fn test_delete_command() {
        let task_id = Uuid::new_v4();
        let args = Args::parse_from(["to-not-do", "delete", &task_id.to_string()]);
        if let Commands::Delete { task_id: id } = args.command {
            assert_eq!(id, task_id);
        } else {
            panic!("Expected Delete command");
        }
    }

    #[test]
    fn test_list_command_with_filter() {
        let args = Args::parse_from(["to-not-do", "list", "done"]);
        if let Commands::List { filter } = args.command {
            assert_eq!(filter, Some(TaskState::Done));
        } else {
            panic!("Expected List command with filter");
        }
    }

    #[test]
    fn test_list_command_without_filter() {
        let args = Args::parse_from(["to-not-do", "list"]);
        if let Commands::List { filter } = args.command {
            assert_eq!(filter, None);
        } else {
            panic!("Expected List command without filter");
        }
    }

    #[test]
    fn test_mark_done_command() {
        let task_id = Uuid::new_v4();
        let args = Args::parse_from(["to-not-do", "mark-done", &task_id.to_string()]);
        if let Commands::MarkDone { task_id: id } = args.command {
            assert_eq!(id, task_id);
        } else {
            panic!("Expected MarkDone command");
        }
    }

    #[test]
    fn test_mark_in_progress_command() {
        let task_id = Uuid::new_v4();
        let args = Args::parse_from(["to-not-do", "mark-in-progress", &task_id.to_string()]);
        if let Commands::MarkInProgress { task_id: id } = args.command {
            assert_eq!(id, task_id);
        } else {
            panic!("Expected MarkInProgress command");
        }
    }
}
