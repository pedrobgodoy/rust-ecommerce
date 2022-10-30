rundb:
  docker compose up -d

createdb:
  docker compose exec -it db psql -U postgres -c "CREATE DATABASE catalog"

dropdb:
  docker compose exec -it db psql -U postgres -c "DROP DATABASE catalog"

migrate-apply:
  cd ./db && npx prisma migrate deploy

migrate-new:
  cd ./db && npx prisma migrate dev

resetdb:
  cd ./db && npx prisma migrate reset

build:
  cargo build --release

run-dev:
  cargo watch -x run

run-prod: build
  ./target/release/api-01

test:
  cargo test

test-stress:
  docker compose run k6 run /k6/tests/create_item.ts

build-image:
  docker build -t pedrobgodoy/api-01 .

push-image: build-image
  docker push pedrobgodoy/api-01