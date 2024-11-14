use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cli::TaskState;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const DB_FILE_NAME: &str = "task_manager.json";

pub fn create_data_directory(data_dir: &Path) -> PathBuf {
    let app_dir = data_dir.join(APP_NAME);

    if !app_dir.exists() {
        std::fs::create_dir(&app_dir).expect("Failed to create data directory");
    }

    app_dir
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Task {
    id: Uuid,
    description: String,
    state: TaskState,
    created_at: NaiveDate,
    updated_at: NaiveDate,
}

impl Task {
    pub fn new(description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.to_string(),
            state: TaskState::Todo,
            created_at: chrono::Utc::now().date_naive(),
            updated_at: chrono::Utc::now().date_naive(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn state(&self) -> TaskState {
        self.state
    }

    fn created_at(&self) -> NaiveDate {
        self.created_at
    }

    fn updated_at(&self) -> NaiveDate {
        self.updated_at
    }

    fn set_state(&mut self, state: TaskState) {
        self.state = state;
        self.updated_at = chrono::Utc::now().date_naive();
    }

    fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
        self.updated_at = chrono::Utc::now().date_naive();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    name: String,
    version: String,
    tasks: Vec<Task>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            name: APP_NAME.to_string(),
            version: VERSION.to_string(),
            tasks: Vec::new(),
        }
    }
}

pub struct DatabaseManager {
    db_path: PathBuf,
    db: Database,
}

impl DatabaseManager {
    pub fn open(path_to_db: &Path) -> Self {
        if !Self::is_valid_path(path_to_db) {
            return Self::create(path_to_db);
        }

        let db = Self::read(path_to_db);

        Self {
            db_path: path_to_db.to_path_buf(),
            db,
        }
    }

    pub fn contains_task(&mut self, task_id: Uuid) -> bool {
        self.db.tasks.iter().any(|t| t.id == task_id)
    }

    pub fn delete_task(&mut self, task_id: Uuid) {
        self.db.tasks.retain(|t| t.id != task_id);
        Self::save(&self.db_path, &self.db);
    }

    pub fn set_task_state(&mut self, task_id: Uuid, state: TaskState) -> Result<(), ()> {
        if let Some(task) = self.db.tasks.iter_mut().find(|t| t.id == task_id) {
            task.set_state(state);
            Self::save(&self.db_path, &self.db);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_tasks(&mut self) -> &Vec<Task> {
        self.db = Self::read(&self.db_path);

        &self.db.tasks
    }

    pub fn get_task(&mut self, task_id: Uuid) -> Option<&Task> {
        self.db.tasks.iter().find(|t| t.id == task_id)
    }

    pub fn add_task(&mut self, task: &Task) {
        self.db.tasks.push(task.clone());
        Self::save(&self.db_path, &self.db);
    }

    fn read(db_file_path: &Path) -> Database {
        let db_file = File::open(db_file_path).expect("Failed to open database file");

        let reader = std::io::BufReader::new(db_file);

        serde_json::from_reader(reader).expect("Failed to read database file")
    }

    fn save(db_path: &Path, db: &Database) {
        let mut db_file = OpenOptions::new()
            .write(true)
            .open(db_path)
            .expect("Failed to open database file");
        let json_db = serde_json::to_string_pretty(db).expect("Failed to serialize database");

        db_file
            .write_all(&json_db.as_bytes())
            .expect("Failed to write to database file");
    }

    fn is_valid_path(path_to_db: &Path) -> bool {
        path_to_db.exists() && path_to_db.is_file()
    }

    fn create(path: &Path) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Failed to create database file");

        let db = Database::default();

        serde_json::to_writer(&file, &db).expect("Failed to write to database file");

        Self {
            db_path: path.to_path_buf(),
            db,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn test_create_database() {
        let dir = tempdir().unwrap();

        let data_dir = create_data_directory(dir.path());
        let db_path = data_dir.join(DB_FILE_NAME);

        let mut db_manager = DatabaseManager::open(&db_path);

        assert!(db_path.exists());

        let tasks = db_manager.get_tasks();

        assert!(tasks.is_empty());
    }

    #[test]
    fn test_open_existing_database() {
        let dir = tempdir().unwrap();

        let data_dir = create_data_directory(dir.path());
        let db_path = data_dir.join(DB_FILE_NAME);

        let mut db_manager = DatabaseManager::open(&db_path);

        assert!(db_path.exists());

        let tasks = db_manager.get_tasks();

        assert!(tasks.is_empty());
    }

    #[test]
    fn test_add_task() {
        let dir = tempdir().unwrap();

        let data_dir = create_data_directory(dir.path());
        let db_path = data_dir.join(DB_FILE_NAME);

        let mut db_manager = DatabaseManager::open(&db_path);

        let task = Task {
            id: Uuid::new_v4(),
            description: "New task".to_string(),
            state: TaskState::Todo,
            created_at: Utc::now().date_naive(),
            updated_at: Utc::now().date_naive(),
        };

        db_manager.add_task(&task);

        assert_eq!(db_manager.db.tasks.len(), 1);

        let tasks = db_manager.get_tasks();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0], task);
    }

    #[test]
    fn test_save_and_load_database() {
        let dir = tempdir().unwrap();

        let data_dir = create_data_directory(dir.path());
        let db_path = data_dir.join(DB_FILE_NAME);

        let mut db_manager = DatabaseManager::open(&db_path);

        let task = Task {
            id: Uuid::new_v4(),
            description: "Persistent task".to_string(),
            state: TaskState::Todo,
            created_at: Utc::now().date_naive(),
            updated_at: Utc::now().date_naive(),
        };

        db_manager.add_task(&task);

        let mut db_manager = DatabaseManager::open(&db_path);
        let tasks = db_manager.get_tasks();
        println!("{:?}", tasks.len());
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0], task);
    }

    #[test]
    fn test_create_data_directory() {
        let dir = tempdir().unwrap();

        let data_dir = create_data_directory(dir.path());

        assert!(data_dir.exists());
        assert!(data_dir.is_dir());
    }

    #[test]
    fn test_add_multiple_tasks() {
        let dir = tempdir().unwrap();

        let data_dir = create_data_directory(dir.path());
        let db_path = data_dir.join(DB_FILE_NAME);

        let mut db_manager = DatabaseManager::open(&db_path);

        for i in 0..100 {
            let task = Task {
                id: Uuid::new_v4(),
                description: format!("Task {}", i),
                state: TaskState::Todo,
                created_at: Utc::now().date_naive(),
                updated_at: Utc::now().date_naive(),
            };

            db_manager.add_task(&task);
        }

        let tasks = db_manager.get_tasks();

        assert_eq!(tasks.len(), 100);
    }

    #[test]
    fn test_update_task_state() {
        let dir = tempdir().unwrap();

        let data_dir = create_data_directory(dir.path());
        let db_path = data_dir.join(DB_FILE_NAME);

        let mut db_manager = DatabaseManager::open(&db_path);

        let task = Task {
            id: Uuid::new_v4(),
            description: "Task to update".to_string(),
            state: TaskState::Todo,
            created_at: Utc::now().date_naive(),
            updated_at: Utc::now().date_naive(),
        };

        db_manager.add_task(&task);

        let tasks = db_manager.get_tasks();

        let task_id = tasks[0].id;
        db_manager
            .set_task_state(task_id, TaskState::Done)
            .expect("Failed to update task state");

        let new_tasks = db_manager.get_tasks();
        let updated_task = new_tasks.iter().find(|t| t.id == task_id).unwrap();

        assert_eq!(updated_task.state, TaskState::Done);
    }

    #[test]
    fn test_remove_task() {
        let dir = tempdir().unwrap();

        let data_dir = create_data_directory(dir.path());
        let db_path = data_dir.join(DB_FILE_NAME);

        let mut db_manager = DatabaseManager::open(&db_path);

        let task = Task {
            id: Uuid::new_v4(),
            description: "Task to remove".to_string(),
            state: TaskState::Todo,
            created_at: Utc::now().date_naive(),
            updated_at: Utc::now().date_naive(),
        };

        db_manager.add_task(&task);
        assert_eq!(db_manager.db.tasks.len(), 1);

        let tasks = db_manager.get_tasks();
        let task_id = tasks.get(0).expect("Failed to get task").id;

        db_manager.delete_task(task_id);

        assert_eq!(db_manager.db.tasks.len(), 0);
    }

    #[test]
    fn test_load_corrupted_database() {
        let dir = tempdir().unwrap();

        let data_dir = create_data_directory(dir.path());
        let db_path = data_dir.join(DB_FILE_NAME);

        let mut file = File::create(&db_path).unwrap();
        file.write_all(b"corrupted data").unwrap();

        let result = std::panic::catch_unwind(|| {
            DatabaseManager::open(&db_path);
        });

        assert!(result.is_err());
    }
}
