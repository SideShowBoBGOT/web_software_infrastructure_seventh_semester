FROM mysql:8.0
WORKDIR /docker-entrypoint-initdb.d
ADD setup.sql /docker-entrypoint-initdb.d
EXPOSE 3306
