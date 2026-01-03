# Load .env automatically
include .env
export

SQLX := sqlx
DB_URL := $(DATABASE_URL)

.PHONY: help
help:
	@echo ""
	@echo "Available commands:"
	@echo "  make migrate-new name=<migration_name>   Create new migration"
	@echo "  make migrate-run                          Run migrations"
	@echo "  make migrate-revert                       Revert last migration"
	@echo "  make migrate-redo                         Revert + run migrations"
	@echo "  make migrate-info                         Show migration status"
	@echo ""

# ------------------------
# MIGRATIONS
# ------------------------

.PHONY: migrate-new
migrate-new:
	@if [ -z "$(name)" ]; then \
		echo "❌ Migration name is required. Usage:"; \
		echo "   make migrate-new name=add_phones_table"; \
		exit 1; \
	fi
	$(SQLX) migrate add $(name)

.PHONY: migrate-run
migrate-run:
	$(SQLX) migrate run

.PHONY: migrate-revert
migrate-revert:
	$(SQLX) migrate revert

.PHONY: migrate-redo
migrate-redo:
	$(SQLX) migrate revert
	$(SQLX) migrate run

.PHONY: migrate-info
migrate-info:
	$(SQLX) migrate info
