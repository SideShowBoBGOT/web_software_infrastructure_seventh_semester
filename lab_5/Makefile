.PHONY: all clean

all:
	docker-compose up --build

restart:
	docker-compose restart

clean: DOCKER_CONTAINERS = $(shell docker ps -qa)
clean: DOCKER_IMAGES = $(shell docker images -q)
clean: DOCKER_NETWORKS = $(shell docker network ls -q)
clean:
	docker-compose down -v