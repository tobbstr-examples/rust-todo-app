use super::model::ListTodosItem;
use super::model::Status;
use super::model::Task;
use super::model::Todo;
use crate::shared;
use deadpool_postgres::Client;
use deadpool_postgres::PoolError;
use uuid::Uuid;

pub trait Storage {
    async fn list_todos(&self, cli: Client) -> Result<Vec<ListTodosItem>, shared::Error>;
    async fn get_todo_by_id(&self, cli: Client, id: &Uuid) -> Result<Todo, shared::Error>;
    async fn insert_todo(&self, cli: Client, title: &str) -> Result<Todo, shared::Error>;
    async fn insert_task(
        &self,
        cli: Client,
        todo_id: &Uuid,
        title: &str,
        description: &str,
    ) -> Result<Task, shared::Error>;
    async fn update_task(
        &self,
        cli: Client,
        id: Uuid,
        title: &str,
        description: &str,
        status: Status,
    ) -> Result<(), shared::Error>;
    async fn get_db_conn(&self) -> Result<Client, PoolError>;
}

#[derive(Clone)]
pub struct Service<S: Storage> {
    storage: S,
}

impl<S: Storage> Service<S> {
    pub fn new(storage: S) -> Self {
        Service { storage: storage }
    }

    pub async fn get_todo_by_id(&self, id: &uuid::Uuid) -> Result<Todo, shared::Error> {
        let pg_client = match self.storage.get_db_conn().await {
            Ok(client) => client,
            Err(err) => {
                return Err(shared::Error::from_source(
                    Box::new(err),
                    "getting a database connection from the pool".to_string(),
                )
                .add_var("id".to_string(), shared::VarValue::String(id.to_string())));
            }
        };

        self.storage.get_todo_by_id(pg_client, id).await
    }

    pub async fn list_todos(&self) -> Result<Vec<ListTodosItem>, shared::Error> {
        let pg_client = match self.storage.get_db_conn().await {
            Ok(client) => client,
            Err(err) => {
                return Err(shared::Error::from_source(
                    Box::new(err),
                    "getting a database connection from the pool".to_string(),
                ))
            }
        };

        self.storage.list_todos(pg_client).await
    }

    pub async fn create_todo(&self, title: &str) -> Result<Todo, shared::Error> {
        let pg_client = match self.storage.get_db_conn().await {
            Ok(client) => client,
            Err(err) => {
                return Err(shared::Error::from_source(
                    Box::new(err),
                    "getting a database connection from the pool".to_string(),
                ))
            }
        };

        self.storage.insert_todo(pg_client, title).await
    }

    pub async fn create_task(&self, cmd: CreateTaskCommand<'_>) -> Result<Task, shared::Error> {
        let pg_client = match self.storage.get_db_conn().await {
            Ok(client) => client,
            Err(err) => {
                return Err(shared::Error::from_source(
                    Box::new(err),
                    "getting a database connection from the pool".to_string(),
                ))
            }
        };

        return self
            .storage
            .insert_task(pg_client, &cmd.todo_id, cmd.title, cmd.description)
            .await;
    }

    pub async fn complete_task(&self, id: Uuid) -> Result<(), shared::Error> {
        let pg_client = match self.storage.get_db_conn().await {
            Ok(client) => client,
            Err(err) => {
                return Err(shared::Error::from_source(
                    Box::new(err),
                    "getting a database connection from the pool".to_string(),
                ))
            }
        };

        // Check preconditions here
        // ...

        // Mark task completed
        self.storage
            .update_task(pg_client, id, "", "", Status::Completed)
            .await
    }
}

pub struct CreateTaskCommand<'a> {
    pub todo_id: uuid::Uuid,
    pub title: &'a str,
    pub description: &'a str,
}
