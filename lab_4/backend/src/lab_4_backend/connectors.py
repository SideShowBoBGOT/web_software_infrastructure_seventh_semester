import mysql.connector
from pymongo import MongoClient
import os

MYSQL_CREDENTIALS = {
    "host": os.getenv("MYSQL_HOST", "mysql"),
    "port": os.getenv("MYSQL_PORT", "3306"),
    "user": os.getenv("MYSQL_USER", "root"),
    "password": os.getenv("MYSQL_PASSWORD", "root"),
    "database": os.getenv("MYSQL_DATABASE", "lab4DB")
}

MONGO_CREDENTIALS = os.getenv("MONGO_URI", "mongodb://mongodb:27017")

def mysql_conn():
    return mysql.connector.connect(
        host=MYSQL_CREDENTIALS["host"],
        user=MYSQL_CREDENTIALS["user"],
        port=MYSQL_CREDENTIALS["port"],
        password=MYSQL_CREDENTIALS["password"],
        database=MYSQL_CREDENTIALS["database"]
    )

def mongo_conn() -> MongoClient[dict[str, str]]:
    return MongoClient(MONGO_CREDENTIALS)
