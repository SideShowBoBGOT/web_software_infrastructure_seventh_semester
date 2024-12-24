ifneq (,$(wildcard ./.env))
    include .env
    export $(shell sed 's/=.*//' .env)
endif

REPO_ROOT:=$(CURDIR)/..

.PHONY: all down restart clean rebuild print-env total-clean

all: build

total-clean: DOCKER_CONTAINERS = $(shell docker ps -qa)
total-clean: DOCKER_IMAGES = $(shell docker images -q)
total-clean: DOCKER_NETWORKS = $(shell docker network ls -q)
total-clean: DOCKER_VOLUMES = $(shell docker volume ls -q)
total-clean:
	-@if [ -n "$(DOCKER_CONTAINERS)" ]; then \
		docker stop $(DOCKER_CONTAINERS); \
		docker rm -f $(DOCKER_CONTAINERS); \
	fi

	-@if [ -n "$(DOCKER_IMAGES)" ]; then \
		docker rmi -f $(DOCKER_IMAGES); \
	fi

	-@if [ -n "$(DOCKER_NETWORKS)" ]; then \
		docker network rm -f $(DOCKER_NETWORKS); \
	fi

	-@if [ -n "$(DOCKER_VOLUMES)" ]; then \
		docker volume rm -f $(DOCKER_VOLUMES); \
	fi

print-env:
	echo BACKEND_PORT is $(BACKEND_PORT)

$(IMAGES_EXTERNAL_PATH):
	mkdir -f $(IMAGES_EXTERNAL_PATH)

build: | $(IMAGES_EXTERNAL_PATH)
	chmod -R 755 $(REPO_ROOT)
	docker-compose up --build

down:
	docker-compose down

clean:
	rm -rf $(IMAGES_EXTERNAL_PATH)
	docker-compose down -v --rmi all
	docker system prune -f

rebuild: clean build