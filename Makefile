rundb:
	docker compose up -d

createdb:
	docker compose exec -it db psql -U postgres -c "CREATE DATABASE catalog"

dropdb:
	docker compose exec -it db psql -U postgres -c "DROP DATABASE catalog"

.PHONY: rundb createdb dropdb