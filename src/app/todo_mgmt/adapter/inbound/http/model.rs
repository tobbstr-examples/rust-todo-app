use crate::app::todo_mgmt::model;
use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ListTodosItem {
    pub id: Uuid,
    pub title: String,
    pub status: String,
    pub task_count: u16,
}

#[derive(Serialize)]
pub struct ListTodosResponse {
    pub data: Vec<ListTodosItem>,
}

impl Responder for ListTodosResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(&self)
    }
}

impl From<Vec<model::ListTodosItem>> for ListTodosResponse {
    fn from(items: Vec<model::ListTodosItem>) -> Self {
        let todos = items
            .into_iter()
            .map(|item| ListTodosItem {
                id: item.id,
                title: item.title,
                status: item.status.to_string(),
                task_count: item.task_count,
            })
            .collect();
        ListTodosResponse { data: todos }
    }
}

#[derive(Serialize)]
pub struct GetTodoResponse {
    pub data: Todo,
}

impl Responder for GetTodoResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(&self)
    }
}

impl From<model::Todo> for GetTodoResponse {
    fn from(todo: model::Todo) -> Self {
        GetTodoResponse {
            data: Todo {
                id: todo.id.to_string(),
                title: todo.title,
                status: todo.status.to_string(),
                tasks: todo
                    .tasks
                    .into_iter()
                    .map(|task| Task {
                        id: task.id,
                        title: task.title,
                        description: task.description,
                        status: task.status.to_string(),
                        created_at: task.created_at.to_string(),
                        updated_at: task.updated_at.to_string(),
                    })
                    .collect(),
                created_at: todo.created_at.to_string(),
                updated_at: todo.updated_at.to_string(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub status: String,
    pub tasks: Vec<Task>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

#[derive(Serialize)]
pub struct CreateTodoResponse {
    pub data: Todo,
}

impl From<model::Todo> for CreateTodoResponse {
    fn from(value: model::Todo) -> Self {
        CreateTodoResponse {
            data: Todo {
                id: value.id.to_string(),
                title: value.title,
                status: value.status.to_string(),
                tasks: vec![],
                created_at: value.created_at.to_string(),
                updated_at: value.updated_at.to_string(),
            },
        }
    }
}

impl Responder for CreateTodoResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    pub data: Task,
}

impl From<model::Task> for CreateTaskResponse {
    fn from(value: model::Task) -> Self {
        CreateTaskResponse {
            data: Task {
                id: value.id,
                title: value.title,
                description: value.description,
                status: value.status.to_string(),
                created_at: value.created_at.to_string(),
                updated_at: value.updated_at.to_string(),
            },
        }
    }
}

impl Responder for CreateTaskResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
