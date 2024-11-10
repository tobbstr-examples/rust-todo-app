--! list_todos
select td.id,                                td.title,                  td.status,
       td.created_at,                        td.updated_at,             count(tsk.id) as task_count
from todo.todo td
left join todo.task tsk on td.id = tsk.todo_id
group by td.id
order by td.created_at desc;

--! get_todo_by_id
select id, title, "status", created_at, updated_at from todo.todo where id = :id;

--! get_tasks_by_todo_id
select id, title, "description", "status", created_at, updated_at from todo.task where todo_id = :todo_id;
