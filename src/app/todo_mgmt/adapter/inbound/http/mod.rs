pub mod model;

use crate::app::todo_mgmt::port::CreateTaskCommand;
use crate::app::todo_mgmt::port::Service;
use crate::app::todo_mgmt::port::Storage;
use actix_web::HttpResponse;
use actix_web::{error, web, Responder};
use model::CreateTaskRequest;
use model::CreateTaskResponse;
use model::CreateTodoRequest;
use model::CreateTodoResponse;
use model::GetTodoResponse;
use model::ListTodosResponse;

#[derive(Clone)]
pub struct Controller<S: Storage> {
    service: Service<S>,
}

impl<S: Storage> Controller<S> {
    pub fn new(svc: Service<S>) -> Self {
        Controller { service: svc }
    }

    pub async fn list_todos(&self) -> Result<impl Responder, actix_web::Error> {
        let todos = self.service.list_todos().await;
        match todos {
            Ok(todos) => Ok(ListTodosResponse::from(todos)),
            Err(err) => {
                println!("Error: {:?}", err);
                Err(error::ErrorInternalServerError(err))
            }
        }
    }

    pub async fn get_todo_by_id(&self, id: uuid::Uuid) -> Result<impl Responder, actix_web::Error> {
        let todo = self.service.get_todo_by_id(&id).await;
        match todo {
            Ok(todo) => Ok(GetTodoResponse::from(todo)),
            Err(err) => {
                println!("Error: {:?}", err);
                Err(error::ErrorInternalServerError(err))
            }
        }
    }

    pub async fn create_todo(
        &self,
        req: web::Json<CreateTodoRequest>,
    ) -> Result<impl Responder, actix_web::Error> {
        let todo = self.service.create_todo(&req.title).await;
        match todo {
            Ok(todo) => Ok(CreateTodoResponse::from(todo)),
            Err(err) => {
                println!("Error: {:?}", err);
                Err(error::ErrorInternalServerError(err))
            }
        }
    }

    pub async fn create_task(
        &self,
        req: web::Json<CreateTaskRequest>,
        path: web::Path<uuid::Uuid>,
    ) -> Result<impl Responder, actix_web::Error> {
        let task = self
            .service
            .create_task(CreateTaskCommand {
                todo_id: path.into_inner(),
                title: &req.title,
                description: &req.description,
            })
            .await;
        match task {
            Ok(task) => Ok(CreateTaskResponse::from(task)),
            Err(err) => {
                println!("Error: {:?}", err);
                Err(error::ErrorInternalServerError(err))
            }
        }
    }

    pub async fn complete_task(
        &self,
        _todo_id: uuid::Uuid,
        task_id: uuid::Uuid,
    ) -> Result<impl Responder, actix_web::Error> {
        match self.service.complete_task(task_id).await {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(err) => {
                println!("Error: {:?}", err);
                Err(error::ErrorInternalServerError(err))
            }
        }
    }
}
