import mysql.connector
from pymongo import MongoClient

MYSQL_CREDENTIALS = {
    "host": "host.docker.internal",
    "port": "3306",
    "user": "root",
    "password": "my-secret-pw",
    "database": "lab4DB"
}

MONGO_CREDENTIALS = "mongodb://host.docker.internal:27017"

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
