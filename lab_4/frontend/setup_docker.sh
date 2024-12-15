docker build -t lab4-frontend:latest .
docker run -d -p 55001:55001 lab4-frontend:latest
# if you need to rebuild after making changes:
# docker build --no-cache -t lab4-frontend:latest .