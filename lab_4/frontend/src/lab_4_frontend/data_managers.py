import requests
from typing import Any
import json
from marshmallow import Schema, fields
from lab_4_frontend.misc import cast_to_schema

GROUPS_ADDRESS = "http://backend:55002/groups/"
STUDENTS_ADDRESS = "http://backend:55002/students/"
SCHEDULE_ADDRESS = "http://backend:55002/schedule/"

def get_leaders() -> list[Any]:
    return [group[2] for group in get_groups()]

class StudentSchema(Schema):
    id = fields.Integer(required=True)
    groupId = fields.Integer(required=True)
    name = fields.String(required=True)
    surname = fields.String(required=True)

class GroupSchema(Schema):
    id = fields.Integer(required=True)
    name = fields.String(required=True)

STUDENT_SCHEMA = StudentSchema()
STUDENTS_SCHEMA = StudentSchema(many=True)
GROUP_SCHEMA = GroupSchema()
GROUPS_SCHEMA = GroupSchema(many=True)

def get_groups() -> list[Any]:
    response = requests.post(GROUPS_ADDRESS + "get/")
    return cast_to_schema(GROUPS_SCHEMA, response.json()[0]["data"])

def get_students() -> list[Any]:
    response = requests.post(STUDENTS_ADDRESS + "get/")
    return cast_to_schema(STUDENTS_SCHEMA, response.json()[0]["data"])

def get_student(student_id: int) -> Any:
    response = requests.post(
        STUDENTS_ADDRESS + "getById/",
        json={"studentId": student_id},
        headers={'Content-Type': 'application/json'}
    )
    return cast_to_schema(STUDENT_SCHEMA, response.json()[0]["data"])

def get_schedule() -> list[Any]:
    return requests.post(SCHEDULE_ADDRESS + "get/").json()[0]["data"]

def addStudent(groupId: int, name: str, surname: str):
    return requests.post(STUDENTS_ADDRESS + "change/add",
        data=json.dumps({
            "groupId": groupId,
            "name": name,
            "surname": surname
        }),  
        headers={'Content-Type': 'application/json'}
    )

def deleteStudent(studentId: int):
    return requests.post(STUDENTS_ADDRESS + "change/delete",
        data=json.dumps({
            "studentId": studentId
        }),  
        headers={'Content-Type': 'application/json'}
    )

def changeGroup(studentId: int, newGroupId: int):
    return requests.post(STUDENTS_ADDRESS + "change/group",
        data=json.dumps({
            "studentId": studentId,
            "groupId": newGroupId
        }),  
        headers={'Content-Type': 'application/json'}
    )

