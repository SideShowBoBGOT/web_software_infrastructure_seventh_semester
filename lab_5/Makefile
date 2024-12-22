.PHONY: all clean

all:
	docker-compose up --build

restart:
	docker-compose restart

clean: DOCKER_CONTAINERS = $(shell docker ps -qa)
clean: DOCKER_IMAGES = $(shell docker images -q)
clean:
	@if [ -n "$(DOCKER_CONTAINERS)" ]; then \
		docker stop $(DOCKER_CONTAINERS); \
		docker rm $(DOCKER_CONTAINERS); \
	fi

	@if [ -n "$(DOCKER_IMAGES)" ]; then \
		docker rmi $(DOCKER_IMAGES) -f; \
	fi