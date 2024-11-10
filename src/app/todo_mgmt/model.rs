use crate::app::todo_mgmt::adapter::outbound::pg::types::todo::Status as PgStatus;
use uuid::Uuid;

pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub status: Status,
    pub tasks: Vec<Task>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

pub struct ListTodosItem {
    pub id: Uuid,
    pub title: String,
    pub status: Status,
    pub task_count: u16,
}

pub enum Status {
    Created,
    Completed,
    Deleted,
    Updated,
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Created => "created".to_string(),
            Status::Completed => "completed".to_string(),
            Status::Deleted => "deleted".to_string(),
            Status::Updated => "updated".to_string(),
        }
    }
}

impl From<PgStatus> for Status {
    fn from(value: PgStatus) -> Self {
        match value {
            PgStatus::created => Status::Created,
            PgStatus::completed => Status::Completed,
            PgStatus::deleted => Status::Deleted,
            PgStatus::updated => Status::Updated,
        }
    }
}
