mod cli;
mod error;
mod file_management;

use clap::Parser;
use cli::{Args, Commands};

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Add { task_description } => {
            println!("Adding task: {}", task_description);
        }
        Commands::Update { task_id } => {
            println!("Updating task: {}", task_id);
        }
        Commands::Delete { task_id } => {
            println!("Deleting task: {}", task_id);
        }
        Commands::List { filter } => {
            println!("Listing tasks with filter: {:?}", filter);
        }
        Commands::MarkDone { task_id } => {
            println!("Marking task as done: {}", task_id);
        }
        Commands::MarkInProgress { task_id } => {
            println!("Marking task as in progress: {}", task_id);
        }
    }
}
