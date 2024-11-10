--! insert_todo
insert into todo.todo (title) values (:title) returning id, title, "status", created_at, updated_at;

--! insert_task
insert into todo.task (
    todo_id,  title,  "description"
)
values (
    :todo_id, :title, :description
) returning *;

--! update_task
update todo.task set
    title = coalesce(nullif(:title, ''), title),
    "description" = coalesce(nullif(:description, ''), "description"),
    "status" = coalesce(:status, "status"),
    updated_at = now()
where id = :id
returning *;
