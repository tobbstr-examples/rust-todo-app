create schema if not exists todo;
grant usage on schema todo to public;

CREATE EXTENSION IF NOT EXISTS "pgcrypto"; -- for generating UUIDs

create type todo.status as enum ('created', 'updated', 'completed', 'deleted');

create table if not exists todo.todo (
  id uuid primary key default gen_random_uuid(),
  title text not null,
  "status" todo.status not null default 'created',

  -- Row metadata
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table if not exists todo.task (
  id uuid primary key default gen_random_uuid(),
  todo_id uuid not null references todo.todo(id),
  title text not null,
  "description" text not null,
  "status" todo.status not null default 'created',

  -- Row metadata
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

