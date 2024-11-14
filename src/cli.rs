use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use uuid::{self, Uuid};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
#[command(rename_all = "kebab-case")]
pub enum Commands {
    Add { task_description: String },
    Update { task_id: Uuid },
    Delete { task_id: Uuid },
    List { filter: Option<TaskState> },
    MarkDone { task_id: Uuid },
    MarkInProgress { task_id: Uuid },
}

#[derive(Debug, ValueEnum, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskState {
    Todo,
    InProgress,
    Done,
}
