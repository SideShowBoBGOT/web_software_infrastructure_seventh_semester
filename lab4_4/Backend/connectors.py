import mysql.connector
from pymongo import MongoClient

import config

def mysql_conn():
    return mysql.connector.connect(
        host = config.MYSQL_CREDENTIALS["host"],
        user = config.MYSQL_CREDENTIALS["user"],
        port = config.MYSQL_CREDENTIALS["port"],
        password = config.MYSQL_CREDENTIALS["password"],
        database = config.MYSQL_CREDENTIALS["database"]
    )

def mongo_conn():
    return MongoClient(config.MONGO_CREDENTIALS)