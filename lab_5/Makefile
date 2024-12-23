.PHONY: all down restart clean rebuild dev build-no-cache

all: build

# Normal build with cache
build:
	docker-compose up --build

# Clean build without any cache
build-no-cache:
	docker-compose build --no-cache
	docker-compose up

# Just restart containers
restart:
	docker-compose restart

# Stop and remove containers
down:
	docker-compose down

# Clean everything
clean:
	docker-compose down -v --rmi all
	docker system prune -f

# Complete rebuild
rebuild: clean build

# For frontend changes only
rebuild-frontend:
	docker-compose build --no-cache frontend
	docker-compose up --build frontend

# For backend changes only
rebuild-backend:
	docker-compose build --no-cache backend
	docker-compose up --build backend