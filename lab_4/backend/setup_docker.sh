docker build -t lab4-backend:latest .
docker run -d -p 55002:55002 lab4-backend:latest
# if you need to rebuild after making changes:
# docker build --no-cache -t lab4-backend:latest .