rundb:
	docker compose up -d

createdb:
	docker compose exec -it db psql -U postgres -c "CREATE DATABASE catalog"

dropdb:
	docker compose exec -it db psql -U postgres -c "DROP DATABASE catalog"

build:
	cargo build --release

run\:prod: build
	./target/release/api-01

.PHONY: rundb createdb dropdb