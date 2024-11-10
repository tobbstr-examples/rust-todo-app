mod todo_mgmt;

use actix_web::{self as actix, web, Responder};
use todo_mgmt::adapter::outbound::Storage;
use todo_mgmt::port::Service;

pub struct Config {
    pub db_username: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub http_port: u16,
    pub http_host: String,
}

pub struct App {
    config: Config,
    todo_mgmt_http_controller: Controller<Storage>,
}

impl App {
    pub async fn new(cfg: Config) -> Self {
        let pool = create_pool(&cfg).await.unwrap();

        let todo_mgmt_storage = Storage::new(pool);
        let todo_mgmt_service = Service::new(todo_mgmt_storage);
        let todo_mgmt_http_controller = Controller::new(todo_mgmt_service);

        App {
            config: cfg,
            todo_mgmt_http_controller,
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let http_host = self.config.http_host.clone();
        let http_port = self.config.http_port;
        println!("Listening on {}:{}", http_host, http_port);

        // Create a shallow clone so that we can "move" it into the closure.
        let todo_mgmt_http_controller = self.todo_mgmt_http_controller.clone();

        // The HttpServer::new constructor expects a closure of type Fn which doesn't:
        //  1. move captured values again inside the closure
        //  2. mutate captured values
        // This closure is called on each incoming HTTP request, which is why the closure bounds include
        // the static lifetime. This means that we cannot capture any values from the enclosing "run()"
        // method by reference, hence the need to force captured values to be "moved", see the `::new(move ||`
        // code. Since the todo_mgmt_http_controller value is moved into the closure and given (1) in the list
        // above, then we must pass a clone to the web::Data::new constructor.
        actix::HttpServer::new(move || {
            actix::App::new()
                .app_data(web::Data::new(todo_mgmt_http_controller.clone()))
                .service(
                    actix::web::scope("/v1")
                        .route(
                            "/todos/{id}/tasks",
                            actix::web::post().to(handle_create_task),
                        )
                        .route(
                            "/todos/{todo_id}/tasks/{task_id}:complete",
                            actix::web::post().to(handle_complete_task),
                        )
                        .route("/todos/{id}", actix::web::get().to(handle_get_todo_by_id))
                        .route("/todos", actix::web::get().to(handle_list_todos))
                        .route("/todos", actix::web::post().to(handle_create_todo)),
                )
        })
        .bind((http_host, http_port))?
        .run()
        .await
    }
}

/// Connection pool configuration.
use deadpool_postgres::{Config as PgConfig, CreatePoolError, Pool, Runtime};
use todo_mgmt::adapter::inbound::http::model as TodoMgmtModel;
use todo_mgmt::adapter::inbound::http::Controller;
use tokio_postgres::NoTls;
use uuid::Uuid;

async fn create_pool(cfg: &Config) -> Result<Pool, CreatePoolError> {
    let mut pg_cfg = PgConfig::new();
    pg_cfg.user = Some(cfg.db_username.clone());
    pg_cfg.password = Some(cfg.db_password.clone());
    pg_cfg.host = Some(cfg.db_host.clone());
    pg_cfg.port = Some(cfg.db_port);
    pg_cfg.dbname = Some(cfg.db_name.clone());
    let res = pg_cfg.create_pool(Some(Runtime::Tokio1), NoTls);

    // Test the connection to the database and panic if it fails.
    match res {
        Ok(ref pool) => {
            let get_pool_result = pool.get().await;
            match get_pool_result {
                Ok(_) => res,
                Err(err) => {
                    panic!("getting connection from pool: {}", err);
                }
            }
        }
        Err(err) => {
            panic!("creating connection pool: {}", err);
        }
    }
}

async fn handle_list_todos(
    ctlr: web::Data<Controller<Storage>>,
) -> Result<impl Responder, actix_web::Error> {
    ctlr.list_todos().await
}

async fn handle_create_todo(
    ctlr: web::Data<Controller<Storage>>,
    req: web::Json<TodoMgmtModel::CreateTodoRequest>,
) -> Result<impl Responder, actix_web::Error> {
    ctlr.create_todo(req).await
}

async fn handle_get_todo_by_id(
    ctlr: web::Data<Controller<Storage>>,
    path: web::Path<uuid::Uuid>,
) -> Result<impl Responder, actix_web::Error> {
    let id = path.into_inner();
    ctlr.get_todo_by_id(id).await
}

async fn handle_create_task(
    ctlr: web::Data<Controller<Storage>>,
    path: web::Path<uuid::Uuid>,
    req: web::Json<TodoMgmtModel::CreateTaskRequest>,
) -> Result<impl Responder, actix_web::Error> {
    ctlr.create_task(req, path).await
}

async fn handle_complete_task(
    ctlr: web::Data<Controller<Storage>>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<impl Responder, actix_web::Error> {
    let (todo_id, task_id) = path.into_inner();
    ctlr.complete_task(todo_id, task_id).await
}
