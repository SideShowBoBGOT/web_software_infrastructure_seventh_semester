FROM python:3.11
EXPOSE 5001
WORKDIR /app
ADD . .
RUN pip install --no-cache-dir -r requirements.txt
CMD ["python", "app.py"]