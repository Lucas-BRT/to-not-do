use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ToNotDoError {
    #[error("Task not found: {0}")]
    DatabaseError(DatabaseError),
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),
    #[error("Task with UUID {0} already exists")]
    UuidAlreadyExists(Uuid),
}
