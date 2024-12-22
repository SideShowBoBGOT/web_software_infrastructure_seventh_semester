FROM postgres:15
WORKDIR /docker-entrypoint-initdb.d
ADD setup.sql /docker-entrypoint-initdb.d/
EXPOSE ${POSTGRES_PORT}
