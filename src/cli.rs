use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
#[command(rename_all = "kebab-case")]
pub enum Commands {
    Add { task_description: String },
    Update { task_id: u64 },
    Delete { task_id: u64 },
    List { filter: Option<TaskState> },
    MarkDone { task_id: u64 },
    MarkInProgress { task_id: u64 },
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum TaskState {
    Todo,
    InProgress,
    Done,
}
