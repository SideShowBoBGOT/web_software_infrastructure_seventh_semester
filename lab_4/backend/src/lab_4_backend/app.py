from flask import Flask, jsonify, request, Response, make_response
from functools import wraps
from marshmallow import Schema, fields
import connectors
from typing import Any, ParamSpec, Callable, TypeAlias, TypeVar, cast, Optional
from http import HTTPStatus
from mysql.connector.cursor import MySQLCursor
from mysql.connector import MySQLConnection
import logging
import sys

logger = logging.getLogger('backendLogger')
logger.addHandler(logging.StreamHandler(sys.stdout))
logger.setLevel(logging.DEBUG)

app = Flask(__name__)

T = TypeVar('T')

class StudentIdSchema(Schema):
    studentId: fields.Integer = fields.Integer(required=True)

class GroupChangeSchema(Schema):
    studentId: fields.Integer = fields.Integer(required=True)
    groupId: fields.Integer = fields.Integer(required=True)

class StudentAddSchema(Schema):
    groupId: fields.Integer = fields.Integer(required=True)
    name: fields.String = fields.String(required=True)
    surname: fields.String = fields.String(required=True)

STUDENT_ID_SCHEMA = StudentIdSchema()
GROUP_CHANGE_SCHEMA = GroupChangeSchema()
STUDENTS_ADD_SCHEMA = StudentAddSchema()

def cast_to_schema(schema: Schema, json_data: Any) -> Any:
    return cast(Any, schema.load(json_data))

P = ParamSpec('P')
R = TypeVar('R')
JsonDict: TypeAlias = dict[str, Any]

def db_operation(func: Callable[..., T]) -> Callable[..., T]:
    @wraps(func)
    def wrapper(*args: Any, **kwargs: Any) -> T:
        db: Optional[MySQLConnection] = None
        try:
            db = cast(MySQLConnection, connectors.mysql_conn())
            cur: MySQLCursor = db.cursor()
            result: T = func(cur, *args, **kwargs)
            db.commit()
            return result
        except Exception as e:
            if db:
                db.rollback()
            raise e
        finally:
            if db:
                db.close()
    return wrapper

Student = tuple[int, int, str, str]  # id, groupId, name, surname
Group = tuple[int, str]  # id, name

@app.route("/students/get/", methods=["POST"])
def students_get() -> Response:
    @db_operation
    def get_all_students(cursor: MySQLCursor) -> list[Student]:
        cursor.execute("SELECT * FROM students")
        return cast(list[Student], cursor.fetchall())
    try:
        result: list[Student] = get_all_students()
        return make_response(jsonify({"data": result}), HTTPStatus.OK)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/students/getById/", methods=["POST"])
def students_get_by_id() -> Response:
    @db_operation
    def get_student(cursor: MySQLCursor) -> Optional[Student]:
        data = cast_to_schema(STUDENT_ID_SCHEMA, request.get_json())
        cursor.execute("SELECT * FROM students WHERE id = %s", (data['studentId'],))
        return cast(Optional[Student], cursor.fetchone())
    try:
        result: Optional[Student] = get_student()
        if not result:
            return make_response(jsonify({"error": "Student not found"}), HTTPStatus.NOT_FOUND)
        return make_response(jsonify({"data": result}), HTTPStatus.OK)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/students/change/add/", methods=["POST"])
def students_add() -> Response:
    @db_operation
    def add_student(cursor: MySQLCursor) -> None:
        data = cast_to_schema(STUDENTS_ADD_SCHEMA, request.get_json())
        cursor.execute(
            "INSERT INTO students (groupId, name, surname) VALUES (%s, %s, %s)",
            (data['groupId'], data['name'], data['surname'])
        )
    try:
        add_student()
        return make_response(jsonify({"success": True}), HTTPStatus.CREATED)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/students/change/delete/", methods=["POST"])
def students_delete() -> Response:
    @db_operation
    def delete_student(cursor: MySQLCursor) -> None:
        data = cast_to_schema(STUDENT_ID_SCHEMA, request.get_json())
        cursor.execute("DELETE FROM students WHERE id = %s", (data['studentId'],))
    try:
        delete_student()
        return make_response(jsonify({"success": True}), HTTPStatus.OK)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/students/change/group/", methods=["POST"])
def students_change_group() -> Response:
    @db_operation
    def change_group(cursor: MySQLCursor) -> None:
        data = cast_to_schema(GROUP_CHANGE_SCHEMA, request.get_json())
        cursor.execute(
            "UPDATE students SET groupId = %s WHERE id = %s",
            (str(data['groupId'], data['studentId']))
        )
    try:
        change_group()
        return make_response(jsonify({"success": True}), HTTPStatus.OK)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/groups/get/", methods=["POST"])
def groups_get() -> Response:
    @db_operation
    def get_all_groups(cursor: MySQLCursor) -> list[Group]:
        cursor.execute("SELECT * FROM student_groups")
        return cast(list[Group], cursor.fetchall())
    
    try:
        result: list[Group] = get_all_groups()
        return make_response(jsonify({"data": result}), HTTPStatus.OK)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/schedule/get/", methods=["POST"])
def schedule_get() -> Response:
    try:
        db = connectors.mongo_conn()['schedules_db']
        coll = db["schedule_collection"]
        result = coll.find()
        result_list: list[JsonDict] = [dict(doc, _id=str(doc['_id'])) for doc in result]
        logger.debug(f'schedule_get: {result_list}')
        return make_response(jsonify({"data": result_list}), HTTPStatus.OK)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

if __name__ == "__main__":
    app.debug = True
    app.run(host='0.0.0.0', port=55002)