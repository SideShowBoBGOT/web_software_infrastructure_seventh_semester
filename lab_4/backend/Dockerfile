FROM python:3.11
EXPOSE 55002
WORKDIR /app
ADD . .
RUN pip install -e .
CMD ["python", "src/lab_4_backend/app.py"]