pub mod pg;

use super::super::model::ListTodosItem;
use super::super::model::Status;
use super::super::model::Task;
use super::super::model::Todo;
use crate::app::todo_mgmt::port;
use crate::shared;
use deadpool_postgres::Client;
use deadpool_postgres::Pool as PgPool;
use pg::types::todo::Status as PgStatus;

#[derive(Clone)]
pub struct Storage {
    pg_pool: PgPool,
}

impl Storage {
    pub fn new(pg_pool: PgPool) -> Self {
        Storage { pg_pool }
    }
}

impl port::Storage for Storage {
    async fn list_todos(
        &self,
        cli: Client,
    ) -> Result<Vec<crate::app::todo_mgmt::model::ListTodosItem>, crate::shared::Error> {
        let res = pg::queries::for_reading_todos::list_todos()
            .bind(&cli)
            .map(|todo_row| ListTodosItem {
                id: todo_row.id,
                title: todo_row.title.to_string(),
                status: Status::from(todo_row.status),
                task_count: todo_row.task_count as u16,
            })
            .all()
            .await;
        match res {
            Ok(todos) => Ok(todos),
            Err(err) => {
                let code: Option<String> = match err.code() {
                    Some(sql_state) => Some(sql_state.code().to_string()),
                    None => None,
                };
                Err(shared::Error::from_source(
                    Box::new(err),
                    "listing todos from the database".to_string(),
                )
                .add_var(
                    "pg_error_code".to_string(),
                    match code {
                        Some(code) => shared::VarValue::String(code),
                        None => shared::VarValue::String("".to_string()),
                    },
                ))
            }
        }
    }

    async fn get_todo_by_id(
        &self,
        cli: Client,
        id: &uuid::Uuid,
    ) -> Result<crate::app::todo_mgmt::model::Todo, crate::shared::Error> {
        let todo_res = pg::queries::for_reading_todos::get_todo_by_id()
            .bind(&cli, id)
            .map(|row| Todo {
                id: row.id,
                title: row.title.to_string(),
                status: Status::from(row.status),
                tasks: vec![],
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .one()
            .await;

        let mut todo = match todo_res {
            Ok(todo) => todo,
            Err(err) => {
                let code: Option<String> = match err.code() {
                    Some(sql_state) => Some(sql_state.code().to_string()),
                    None => None,
                };
                return Err(shared::Error::from_source(
                    Box::new(err),
                    "getting a todo from the database".to_string(),
                )
                .add_var(
                    "pg_error_code".to_string(),
                    match code {
                        Some(code) => shared::VarValue::String(code),
                        None => shared::VarValue::String("".to_string()),
                    },
                ));
            }
        };

        let get_tasks_res = pg::queries::for_reading_todos::get_tasks_by_todo_id()
            .bind(&cli, id)
            .map(|row| Task {
                id: row.id,
                title: row.title.to_string(),
                description: row.description.to_string(),
                status: Status::from(row.status),
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .all()
            .await;

        let tasks = match get_tasks_res {
            Ok(tasks) => tasks,
            Err(err) => {
                let code: Option<String> = match err.code() {
                    Some(sql_state) => Some(sql_state.code().to_string()),
                    None => None,
                };
                return Err(shared::Error::from_source(
                    Box::new(err),
                    "getting tasks from the database".to_string(),
                )
                .add_var(
                    "pg_error_code".to_string(),
                    match code {
                        Some(code) => shared::VarValue::String(code),
                        None => shared::VarValue::String("".to_string()),
                    },
                ));
            }
        };

        todo.tasks = tasks;
        Ok(todo)
    }

    async fn insert_todo(&self, cli: Client, title: &str) -> Result<Todo, shared::Error> {
        let res = pg::queries::for_writing_todos::insert_todo()
            .bind(&cli, &title)
            .one()
            .await;
        match res {
            Ok(row) => Ok(Todo {
                id: row.id,
                status: row.status.into(),
                title: row.title,
                created_at: row.created_at,
                updated_at: row.updated_at,
                tasks: vec![],
            }),
            Err(err) => {
                let code: Option<String> = match err.code() {
                    Some(sql_state) => Some(sql_state.code().to_string()),
                    None => None,
                };
                Err(shared::Error::from_source(
                    Box::new(err),
                    "creating a todo in the database".to_string(),
                )
                .add_var(
                    "pg_error_code".to_string(),
                    match code {
                        Some(code) => shared::VarValue::String(code),
                        None => shared::VarValue::String("".to_string()),
                    },
                ))
            }
        }
    }

    async fn insert_task(
        &self,
        cli: Client,
        todo_id: &uuid::Uuid,
        title: &str,
        description: &str,
    ) -> Result<Task, shared::Error> {
        let res = pg::queries::for_writing_todos::insert_task()
            .bind(&cli, &todo_id, &title, &description)
            .one()
            .await;

        match res {
            Ok(row) => Ok(Task {
                id: row.id,
                title: row.title,
                description: row.description,
                status: row.status.into(),
                created_at: row.created_at,
                updated_at: row.updated_at,
            }),
            Err(err) => {
                let code: Option<String> = match err.code() {
                    Some(sql_state) => Some(sql_state.code().to_string()),
                    None => None,
                };
                Err(shared::Error::from_source(
                    Box::new(err),
                    "creating a task in the database".to_string(),
                )
                .add_var(
                    "pg_error_code".to_string(),
                    match code {
                        Some(code) => shared::VarValue::String(code),
                        None => shared::VarValue::String("".to_string()),
                    },
                ))
            }
        }
    }

    async fn update_task(
        &self,
        cli: Client,
        id: uuid::Uuid,
        title: &str,
        description: &str,
        status: Status,
    ) -> Result<(), shared::Error> {
        let st = match status {
            Status::Completed => PgStatus::completed,
            Status::Created => PgStatus::created,
            Status::Deleted => PgStatus::deleted,
            Status::Updated => PgStatus::updated,
        };

        let res = pg::queries::for_writing_todos::update_task()
            .bind(&cli, &title, &description, &st, &id)
            .one()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                let code: Option<String> = match err.code() {
                    Some(sql_state) => Some(sql_state.code().to_string()),
                    None => None,
                };
                Err(shared::Error::from_source(
                    Box::new(err),
                    "updating a task in the database".to_string(),
                )
                .add_var(
                    "pg_error_code".to_string(),
                    match code {
                        Some(code) => shared::VarValue::String(code),
                        None => shared::VarValue::String("".to_string()),
                    },
                ))
            }
        }
    }

    async fn get_db_conn(&self) -> Result<Client, deadpool_postgres::PoolError> {
        self.pg_pool.get().await
    }
}
