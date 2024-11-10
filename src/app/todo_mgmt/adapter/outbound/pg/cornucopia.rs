// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { pub mod todo { #[derive( Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum Status { created,updated,completed,deleted,}impl<'a> postgres_types::ToSql for Status
{
    fn
    to_sql(&self, ty: &postgres_types::Type, buf: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>,>
    {
        let s = match *self { Status::created => "created",Status::updated => "updated",Status::completed => "completed",Status::deleted => "deleted",};
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "status" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 4 { return false; }
                variants.iter().all(|v| match &**v
                { "created" => true,"updated" => true,"completed" => true,"deleted" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(&self, ty: &postgres_types::Type, out: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>>
    { postgres_types::__to_sql_checked(self, ty, out) }
} impl<'a> postgres_types::FromSql<'a> for Status
{
    fn from_sql(ty: &postgres_types::Type, buf: &'a [u8],) ->
    Result<Status, Box<dyn std::error::Error + Sync + Send>,>
    {
        match std::str::from_utf8(buf)?
        {
            "created" => Ok(Status::created),"updated" => Ok(Status::updated),"completed" => Ok(Status::completed),"deleted" => Ok(Status::deleted),s =>
            Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "status" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 4 { return false; }
                variants.iter().all(|v| match &**v
                { "created" => true,"updated" => true,"completed" => true,"deleted" => true,_ => false, })
            } _ => false,
        }
    }
} }}#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod for_reading_todos
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct ListTodos
{ pub id : uuid::Uuid,pub title : String,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,pub task_count : i64,}pub struct ListTodosBorrowed<'a> { pub id : uuid::Uuid,pub title : &'a str,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,pub task_count : i64,}
impl<'a> From<ListTodosBorrowed<'a>> for ListTodos
{
    fn from(ListTodosBorrowed { id,title,status,created_at,updated_at,task_count,}: ListTodosBorrowed<'a>) ->
    Self { Self { id,title: title.into(),status,created_at,updated_at,task_count,} }
}pub struct ListTodosQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> ListTodosBorrowed,
    mapper: fn(ListTodosBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> ListTodosQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(ListTodosBorrowed) -> R) ->
    ListTodosQuery<'a,C,R,N>
    {
        ListTodosQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct GetTodoById
{ pub id : uuid::Uuid,pub title : String,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}pub struct GetTodoByIdBorrowed<'a> { pub id : uuid::Uuid,pub title : &'a str,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}
impl<'a> From<GetTodoByIdBorrowed<'a>> for GetTodoById
{
    fn from(GetTodoByIdBorrowed { id,title,status,created_at,updated_at,}: GetTodoByIdBorrowed<'a>) ->
    Self { Self { id,title: title.into(),status,created_at,updated_at,} }
}pub struct GetTodoByIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetTodoByIdBorrowed,
    mapper: fn(GetTodoByIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetTodoByIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetTodoByIdBorrowed) -> R) ->
    GetTodoByIdQuery<'a,C,R,N>
    {
        GetTodoByIdQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct GetTasksByTodoId
{ pub id : uuid::Uuid,pub title : String,pub description : String,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}pub struct GetTasksByTodoIdBorrowed<'a> { pub id : uuid::Uuid,pub title : &'a str,pub description : &'a str,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}
impl<'a> From<GetTasksByTodoIdBorrowed<'a>> for GetTasksByTodoId
{
    fn from(GetTasksByTodoIdBorrowed { id,title,description,status,created_at,updated_at,}: GetTasksByTodoIdBorrowed<'a>) ->
    Self { Self { id,title: title.into(),description: description.into(),status,created_at,updated_at,} }
}pub struct GetTasksByTodoIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetTasksByTodoIdBorrowed,
    mapper: fn(GetTasksByTodoIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetTasksByTodoIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetTasksByTodoIdBorrowed) -> R) ->
    GetTasksByTodoIdQuery<'a,C,R,N>
    {
        GetTasksByTodoIdQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn list_todos() -> ListTodosStmt
{ ListTodosStmt(cornucopia_async::private::Stmt::new("select td.id,                                td.title,                  td.status,
       td.created_at,                        td.updated_at,             count(tsk.id) as task_count
from todo.todo td
left join todo.task tsk on td.id = tsk.todo_id
group by td.id
order by td.created_at desc")) } pub struct
ListTodosStmt(cornucopia_async::private::Stmt); impl ListTodosStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> ListTodosQuery<'a,C,
ListTodos, 0>
{
    ListTodosQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { ListTodosBorrowed { id: row.get(0),title: row.get(1),status: row.get(2),created_at: row.get(3),updated_at: row.get(4),task_count: row.get(5),} }, mapper: |it| { <ListTodos>::from(it) },
    }
} }pub fn get_todo_by_id() -> GetTodoByIdStmt
{ GetTodoByIdStmt(cornucopia_async::private::Stmt::new("select id, title, \"status\", created_at, updated_at from todo.todo where id = $1")) } pub struct
GetTodoByIdStmt(cornucopia_async::private::Stmt); impl GetTodoByIdStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a uuid::Uuid,) -> GetTodoByIdQuery<'a,C,
GetTodoById, 1>
{
    GetTodoByIdQuery
    {
        client, params: [id,], stmt: &mut self.0, extractor:
        |row| { GetTodoByIdBorrowed { id: row.get(0),title: row.get(1),status: row.get(2),created_at: row.get(3),updated_at: row.get(4),} }, mapper: |it| { <GetTodoById>::from(it) },
    }
} }pub fn get_tasks_by_todo_id() -> GetTasksByTodoIdStmt
{ GetTasksByTodoIdStmt(cornucopia_async::private::Stmt::new("select id, title, \"description\", \"status\", created_at, updated_at from todo.task where todo_id = $1")) } pub struct
GetTasksByTodoIdStmt(cornucopia_async::private::Stmt); impl GetTasksByTodoIdStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
todo_id: &'a uuid::Uuid,) -> GetTasksByTodoIdQuery<'a,C,
GetTasksByTodoId, 1>
{
    GetTasksByTodoIdQuery
    {
        client, params: [todo_id,], stmt: &mut self.0, extractor:
        |row| { GetTasksByTodoIdBorrowed { id: row.get(0),title: row.get(1),description: row.get(2),status: row.get(3),created_at: row.get(4),updated_at: row.get(5),} }, mapper: |it| { <GetTasksByTodoId>::from(it) },
    }
} }}pub mod for_writing_todos
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertTaskParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub todo_id: uuid::Uuid,pub title: T1,pub description: T2,}#[derive( Debug)] pub struct UpdateTaskParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub title: T1,pub description: T2,pub status: super::super::types::todo::Status,pub id: uuid::Uuid,}#[derive( Debug, Clone, PartialEq,)] pub struct InsertTodo
{ pub id : uuid::Uuid,pub title : String,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}pub struct InsertTodoBorrowed<'a> { pub id : uuid::Uuid,pub title : &'a str,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}
impl<'a> From<InsertTodoBorrowed<'a>> for InsertTodo
{
    fn from(InsertTodoBorrowed { id,title,status,created_at,updated_at,}: InsertTodoBorrowed<'a>) ->
    Self { Self { id,title: title.into(),status,created_at,updated_at,} }
}pub struct InsertTodoQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> InsertTodoBorrowed,
    mapper: fn(InsertTodoBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> InsertTodoQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(InsertTodoBorrowed) -> R) ->
    InsertTodoQuery<'a,C,R,N>
    {
        InsertTodoQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct InsertTask
{ pub id : uuid::Uuid,pub todo_id : uuid::Uuid,pub title : String,pub description : String,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}pub struct InsertTaskBorrowed<'a> { pub id : uuid::Uuid,pub todo_id : uuid::Uuid,pub title : &'a str,pub description : &'a str,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}
impl<'a> From<InsertTaskBorrowed<'a>> for InsertTask
{
    fn from(InsertTaskBorrowed { id,todo_id,title,description,status,created_at,updated_at,}: InsertTaskBorrowed<'a>) ->
    Self { Self { id,todo_id,title: title.into(),description: description.into(),status,created_at,updated_at,} }
}pub struct InsertTaskQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> InsertTaskBorrowed,
    mapper: fn(InsertTaskBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> InsertTaskQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(InsertTaskBorrowed) -> R) ->
    InsertTaskQuery<'a,C,R,N>
    {
        InsertTaskQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct UpdateTask
{ pub id : uuid::Uuid,pub todo_id : uuid::Uuid,pub title : String,pub description : String,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}pub struct UpdateTaskBorrowed<'a> { pub id : uuid::Uuid,pub todo_id : uuid::Uuid,pub title : &'a str,pub description : &'a str,pub status : super::super::types::todo::Status,pub created_at : time::OffsetDateTime,pub updated_at : time::OffsetDateTime,}
impl<'a> From<UpdateTaskBorrowed<'a>> for UpdateTask
{
    fn from(UpdateTaskBorrowed { id,todo_id,title,description,status,created_at,updated_at,}: UpdateTaskBorrowed<'a>) ->
    Self { Self { id,todo_id,title: title.into(),description: description.into(),status,created_at,updated_at,} }
}pub struct UpdateTaskQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> UpdateTaskBorrowed,
    mapper: fn(UpdateTaskBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> UpdateTaskQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(UpdateTaskBorrowed) -> R) ->
    UpdateTaskQuery<'a,C,R,N>
    {
        UpdateTaskQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn insert_todo() -> InsertTodoStmt
{ InsertTodoStmt(cornucopia_async::private::Stmt::new("insert into todo.todo (title) values ($1) returning id, title, \"status\", created_at, updated_at")) } pub struct
InsertTodoStmt(cornucopia_async::private::Stmt); impl InsertTodoStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
title: &'a T1,) -> InsertTodoQuery<'a,C,
InsertTodo, 1>
{
    InsertTodoQuery
    {
        client, params: [title,], stmt: &mut self.0, extractor:
        |row| { InsertTodoBorrowed { id: row.get(0),title: row.get(1),status: row.get(2),created_at: row.get(3),updated_at: row.get(4),} }, mapper: |it| { <InsertTodo>::from(it) },
    }
} }pub fn insert_task() -> InsertTaskStmt
{ InsertTaskStmt(cornucopia_async::private::Stmt::new("insert into todo.task (
    todo_id,  title,  \"description\"
)
values (
    $1, $2, $3
) returning *")) } pub struct
InsertTaskStmt(cornucopia_async::private::Stmt); impl InsertTaskStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
todo_id: &'a uuid::Uuid,title: &'a T1,description: &'a T2,) -> InsertTaskQuery<'a,C,
InsertTask, 3>
{
    InsertTaskQuery
    {
        client, params: [todo_id,title,description,], stmt: &mut self.0, extractor:
        |row| { InsertTaskBorrowed { id: row.get(0),todo_id: row.get(1),title: row.get(2),description: row.get(3),status: row.get(4),created_at: row.get(5),updated_at: row.get(6),} }, mapper: |it| { <InsertTask>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
InsertTaskParams<T1,T2,>, InsertTaskQuery<'a, C,
InsertTask, 3>, C> for InsertTaskStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertTaskParams<T1,T2,>) -> InsertTaskQuery<'a, C,
    InsertTask, 3>
    { self.bind(client, &params.todo_id,&params.title,&params.description,) }
}pub fn update_task() -> UpdateTaskStmt
{ UpdateTaskStmt(cornucopia_async::private::Stmt::new("update todo.task set
    title = coalesce(nullif($1, ''), title),
    \"description\" = coalesce(nullif($2, ''), \"description\"),
    \"status\" = coalesce($3, \"status\"),
    updated_at = now()
where id = $4
returning *")) } pub struct
UpdateTaskStmt(cornucopia_async::private::Stmt); impl UpdateTaskStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
title: &'a T1,description: &'a T2,status: &'a super::super::types::todo::Status,id: &'a uuid::Uuid,) -> UpdateTaskQuery<'a,C,
UpdateTask, 4>
{
    UpdateTaskQuery
    {
        client, params: [title,description,status,id,], stmt: &mut self.0, extractor:
        |row| { UpdateTaskBorrowed { id: row.get(0),todo_id: row.get(1),title: row.get(2),description: row.get(3),status: row.get(4),created_at: row.get(5),updated_at: row.get(6),} }, mapper: |it| { <UpdateTask>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
UpdateTaskParams<T1,T2,>, UpdateTaskQuery<'a, C,
UpdateTask, 4>, C> for UpdateTaskStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpdateTaskParams<T1,T2,>) -> UpdateTaskQuery<'a, C,
    UpdateTask, 4>
    { self.bind(client, &params.title,&params.description,&params.status,&params.id,) }
}}}