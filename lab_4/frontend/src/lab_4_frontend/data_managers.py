import requests
from typing import Any
import json
import logging
import sys

GROUPS_ADDRESS = "http://backend:55002/groups/"
STUDENTS_ADDRESS = "http://backend:55002/students/"
SCHEDULE_ADDRESS = "http://backend:55002/schedule/"

logger = logging.getLogger('frontLogger')
logger.addHandler(logging.StreamHandler(sys.stdout))
logger.setLevel(logging.DEBUG)

def get_leaders() -> list[Any]:
    data = [group[2] for group in get_groups()]
    logger.debug(f'get_leaders: {data}')
    return data

def get_groups() -> list[Any]:
    response = requests.post(GROUPS_ADDRESS + "get/")
    data = response.json()["data"]
    logger.debug(f'get_groups: {data}')
    return data

def get_students() -> list[Any]:
    response = requests.post(STUDENTS_ADDRESS + "get/")
    data = response.json()["data"]
    logger.debug(f'get_students: {data}')
    return data

def get_student(student_id: int) -> Any:
    response = requests.post(
        STUDENTS_ADDRESS + "getById/",
        json={"studentId": student_id},
        headers={'Content-Type': 'application/json'}
    )
    data = response.json()["data"]
    logger.debug(f'get_student: {data}')
    return data

def get_schedule() -> list[Any]:
    return requests.post(SCHEDULE_ADDRESS + "get/").json()["data"]

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

