import requests
from typing import Dict, List
import json 

GROUPS_ADRESS = "http://backend:5001/groups/"
STUDENTS_ADRESS = "http://backend:5001/students/"
SCHEDULE_ADRESS = "http://backend:5001/schedule/"

# GROUPS_ADRESS = "localhost:5001/groups/"
# STUDENTS_ADRESS = "localhost:5001/students/"
# SCHEDULE_ADRESS = "localhost:5001/schedule/"


""" GROUPS """

def get_groups() -> List[Dict]:
    # print("Get groups returned", requests.post(GROUPS_ADRESS + "get/").json()[0]["data"])
    return requests.post(GROUPS_ADRESS + "get/").json()[0]["data"]

def get_leaders() -> List[Dict]:
    return [group[2] for group in get_groups()]


""" STUDENTS """

def get_students() -> List[Dict]:
    return requests.post(STUDENTS_ADRESS + "get/").json()[0]["data"]

def get_student(id) -> List[Dict]:
    # print("got student", requests.post(STUDENTS_ADRESS + "getById/",
    #     data=json.dumps(
    #     {
    #         "studentId": id
    #     }),  
    #     headers={'Content-Type': 'application/json'}
    # ))

    return requests.post(STUDENTS_ADRESS + "getById/",
        data=json.dumps(
        {
            "studentId": id
        }),  
        headers={'Content-Type': 'application/json'}
    ).json()[0]["data"]

def addStudent(groupId, name, surname):
    return requests.post(STUDENTS_ADRESS + "change/add",
        data=json.dumps(
        {
            "groupId": groupId,
            "name": name,
            "surname": surname
        }
        ),  
        headers={'Content-Type': 'application/json'}
    )

def deleteStudent(studentId):
    return requests.post(STUDENTS_ADRESS + "change/delete",
        data=json.dumps({
            "studentId": studentId
        }),  
        headers={'Content-Type': 'application/json'}
    )

def changeGroup(studentId, newGroupId):
    return requests.post(STUDENTS_ADRESS + "change/group",
        data=json.dumps({
            "studentId": studentId,
            "groupId": newGroupId
        }),  
        headers={'Content-Type': 'application/json'}
    )


""" SCHEDULE """

def get_schedule() -> List[Dict]:
    return requests.post(SCHEDULE_ADRESS + "get/").json()[0]["data"]