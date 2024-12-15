from flask import Flask, render_template, jsonify, request

import test_data as td
import data_managers as dm

app = Flask(__name__)
DEBUG = False

@app.route("/")
def indexRoute():
    if not DEBUG:
        groups = dm.get_groups()
        students = dm.get_students()
        leaders = dm.get_leaders()
    else:
        groups = td.groups
        students = td.students
        leaders = td.leaders
    return render_template("index.html",
                           groups=groups,
                           students=students,
                           leaders=leaders)

@app.route("/schedule/")
def scheduleRoute():
    if not DEBUG:
        groups = dm.get_groups()
        schedule = dm.get_schedule()
    else:
        groups = td.groups
        schedule = td.schedule
    return render_template("schedule.html",
                           groups=groups,
                           schedule=schedule)

@app.route("/transfer/")
def transferRoute():
    if not DEBUG:
        groups = dm.get_groups()
        student = dm.get_student(request.args.get("studentId"))
        leaders = dm.get_leaders()
    else:
        groups = td.groups
        students = td.students
        leaders = td.leaders
        
        student = None
        for s in students:
            if s["id"] == int(request.args.get("studentId")):
                student = s
                break
    return render_template("transfer.html",
                           groups=groups,
                           student=student,
                           leaders=leaders)

@app.route("/transfer/changeGroup/", methods=["POST"])
def transferChangeGroup():
    data = request.json
    if not DEBUG:
        dm.changeGroup(data["studentId"], data["newGroup"])
        return jsonify(success=True)
    else:
        students = td.students
        for student in students:
            if student["id"] == int(data["studentId"]):
                student["groupId"] = int(data["newGroup"])
                break
        return jsonify(success=True)

@app.route("/addStudent/")
def addStudentRoute():
    if not DEBUG:
        groups = dm.get_groups()
    else:
        groups = td.groups
    return render_template("addStudent.html",
                           groups=groups)

@app.route("/addStudent/commit", methods=["POST"])
def addStudentCommitRoute():
    data = request.json
    if not DEBUG:
        dm.addStudent(
            data["group"][:6],
            data["name"],
            data["surname"]
        )
        return jsonify(success=True)
    else:
        students = td.students
        students.append({
            "id": 0,
            "groupId": data["group"][:6],
            "name": data["name"],
            "surname": data["surname"]
        })
        return jsonify(success=True)

@app.route("/deleteStudent/commit", methods=["POST"])
def deleteStudentCommitRoute():
    data = request.json
    if not DEBUG:
        dm.deleteStudent(data["id"])
        return jsonify(success=True)
    else:
        students = td.students
        for i in range(len(students)):
            if students[i]["id"] == int(data["id"]):
                del students[i]
                break
        return jsonify(success=True)


if __name__ == "__main__":
    app.debug=True
    app.run(host='0.0.0.0', port=55001)