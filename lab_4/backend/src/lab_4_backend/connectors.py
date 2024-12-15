import mysql.connector
from pymongo import MongoClient
import os
import logging

# Set up logging
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

MYSQL_CREDENTIALS = {
    "host": os.getenv("MYSQL_HOST", "mysql"),
    "port": os.getenv("MYSQL_PORT", "3306"),
    "user": os.getenv("MYSQL_USER", "root"),
    "password": os.getenv("MYSQL_PASSWORD", "root"),
    "database": os.getenv("MYSQL_DATABASE", "lab4DB")
}
MONGO_CREDENTIALS = os.getenv("MONGO_URI", "mongodb://mongodb:27017")

def mysql_conn():
    try:
        logger.debug(f"Attempting MySQL connection with credentials: {MYSQL_CREDENTIALS}")
        connection = mysql.connector.connect(
            host=MYSQL_CREDENTIALS["host"],
            user=MYSQL_CREDENTIALS["user"],
            port=MYSQL_CREDENTIALS["port"],
            password=MYSQL_CREDENTIALS["password"],
            database=MYSQL_CREDENTIALS["database"]
        )
        logger.debug("MySQL connection successful")
        return connection
    except mysql.connector.Error as e:
        logger.error(f"MySQL connection failed: {e}")
        raise

def mongo_conn() -> MongoClient[dict[str, str]]:
    try:
        logger.debug(f"Attempting MongoDB connection with URI: {MONGO_CREDENTIALS}")
        client = MongoClient(MONGO_CREDENTIALS)
        
        # Test the connection
        db = client['schedules_db']
        count = db.schedule_collection.count_documents({})
        logger.debug(f"MongoDB connection successful. Found {count} documents in schedule_collection")
        
        # Print all collections in the database
        collections = db.list_collection_names()
        logger.debug(f"Collections in database: {collections}")
        
        return client
    except Exception as e:
        logger.error(f"MongoDB connection failed: {e}")
        raise