mod cli;
mod error;
mod file_management;

use clap::Parser;
use cli::{handle_commands, Args};
use file_management::{create_data_directory, DB_FILE_NAME};

fn main() {
    let base_dir = dirs::data_dir().expect("Failed to get data directory");
    let data_dir = create_data_directory(&base_dir);
    let db_file = data_dir.join(DB_FILE_NAME);

    let mut db_manager = file_management::DatabaseManager::open(&db_file);

    let args = Args::parse();

    handle_commands(args, &mut db_manager);
}
