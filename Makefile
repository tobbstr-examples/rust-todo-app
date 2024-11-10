:PHONY up
up:
	docker run --rm --name some-postgres -e POSTGRES_PASSWORD=postgres -d -p 5432:5432 postgres
	sleep 2
	PGPASSWORD=postgres psql -U postgres -h localhost -f src/app/todo_mgmt/adapter/outbound/pg/schema.sql

:PHONY down
down:
	docker stop some-postgres

:PHONY psql
psql:
	PGPASSWORD=postgres psql -U postgres -h localhost

:PHONY gen
gen:
	cornucopia -q "src/app/todo_mgmt/adapter/outbound/pg/query" -d "src/app/todo_mgmt/adapter/outbound/pg/cornucopia.rs" schema "src/app/todo_mgmt/adapter/outbound/pg/schema.sql"

:PHONY build
build:
	cargo build

:PHONY run
run:
	RUST_LOG=actix_web=debug cargo run
