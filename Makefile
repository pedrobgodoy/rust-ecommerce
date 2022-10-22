rundb:
	docker compose up -d

createdb:
	docker compose exec -it db psql -U postgres -c "CREATE DATABASE catalog"

dropdb:
	docker compose exec -it db psql -U postgres -c "DROP DATABASE catalog"

migrate\:apply:
	cd ./db && npx prisma migrate deploy

migrate\:new:
	cd ./db && npx prisma migrate dev

resetdb:
	cd ./db && npx prisma migrate reset

build:
	cargo build --release

run\:dev:
	cargo watch -x run

run\:prod: build
	./target/release/api-01

.PHONY: rundb createdb dropdb