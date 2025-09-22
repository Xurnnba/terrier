.PHONY: build up down migrate fresh status logs clean restart setup help

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build all services
	docker-compose build

up: ## Start all services
	docker-compose up -d

down: ## Stop all services
	docker-compose down

migrate: ## Run database migrations
	docker-compose --profile migration run --rm migration

fresh: ## Fresh database (drop all tables and reapply migrations)
	docker-compose --profile migration run --rm migration migration fresh

status: ## Check migration status
	docker-compose --profile migration run --rm migration migration status

logs: ## Show logs for all services
	docker-compose logs -f

clean: ## Remove all containers, volumes, and images
	docker-compose down -v --rmi all

restart: down up ## Restart all services

setup: migrate up ## Run migrations and start services
