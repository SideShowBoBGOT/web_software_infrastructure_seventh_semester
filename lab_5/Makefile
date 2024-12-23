ifneq (,$(wildcard ./.env))
    include .env
    export $(shell sed 's/=.*//' .env)
endif

.PHONY: all down restart clean rebuild print-env

all: build

print-env:
	echo BACKEND_PORT is $(BACKEND_PORT)

$(IMAGES_EXTERNAL_PATH):
	mkdir $(IMAGES_EXTERNAL_PATH)

build: | $(IMAGES_EXTERNAL_PATH)
	docker-compose up --build

down:
	docker-compose down

clean:
	rm -rf $(IMAGES_EXTERNAL_PATH)
	docker-compose down -v --rmi all
	docker system prune -f

rebuild: clean build