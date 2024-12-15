from flask import Flask, render_template, jsonify, request, make_response
from marshmallow import Schema, fields, ValidationError
from http import HTTPStatus
import lab_4_frontend.data_managers as dm
from lab_4_frontend.misc import cast_to_schema
from typing import cast, Any

app = Flask(__name__)

class GroupTransferSchema(Schema):
    studentId = fields.Integer(required=True)
    newGroup = fields.String(required=True)

class StudentAddSchema(Schema):
    group = fields.String(required=True)
    name = fields.String(required=True)
    surname = fields.String(required=True)

class StudentDeleteSchema(Schema):
    id = fields.Integer(required=True)

group_transfer_schema = GroupTransferSchema()
student_add_schema = StudentAddSchema()
student_delete_schema = StudentDeleteSchema()

@app.route("/")
def indexRoute():
    try:
        groups = dm.get_groups()
        students = dm.get_students()
        leaders = dm.get_leaders()
        return render_template(
            "index.html",
            groups=groups,
            students=students,
            leaders=leaders
        )
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/schedule/")
def scheduleRoute():
    try:
        groups = dm.get_groups()
        schedule = dm.get_schedule()
        return render_template(
            "schedule.html",
            groups=groups,
            schedule=schedule
        )
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/transfer/")
def transferRoute():
    try:
        student_id = int(request.args["studentId"])
        groups = dm.get_groups()
        student = dm.get_student(student_id)
        leaders = dm.get_leaders()
        
        if not student:
            return make_response(jsonify({"error": "Student not found"}), HTTPStatus.NOT_FOUND)
            
        return render_template(
            "transfer.html",
            groups=groups,
            student=student,
            leaders=leaders
        )
    except ValueError:
        return make_response(jsonify({"error": "Invalid student ID format"}), HTTPStatus.BAD_REQUEST)
    except KeyError:
        return make_response(jsonify({"error": "Student ID is required"}), HTTPStatus.BAD_REQUEST)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/transfer/changeGroup/", methods=["POST"])
def transferChangeGroup():
    try:
        data = cast_to_schema(group_transfer_schema, request.json)
        dm.changeGroup(data["studentId"], data["newGroup"])
        return make_response(jsonify({"success": True}), HTTPStatus.OK)
    except ValidationError as err:
        return make_response(jsonify({"error": cast(list[Any], err.messages)}), HTTPStatus.BAD_REQUEST)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/addStudent/")
def addStudentRoute():
    try:
        groups = dm.get_groups()
        return render_template("addStudent.html", groups=groups)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)

@app.route("/addStudent/commit", methods=["POST"])
def addStudentCommitRoute():
    try:
        data = cast_to_schema(student_add_schema, request.json)
        dm.addStudent(
            data["group"][:6],
            data["name"],
            data["surname"]
        )
        return make_response(jsonify({"success": True}), HTTPStatus.CREATED)
    except ValidationError as err:
        return make_response(jsonify({"error": err.messages}), HTTPStatus.BAD_REQUEST)
    except Exception as e:
        return make_response(jsonify({"error": str(e)}), HTTPStatus.INTERNAL_SERVER_ERROR)


@app.route("/deleteStudent/commit", methods=["POST"])
def deleteStudentCommitRoute():
    try:
        data = cast_to_schema(student_delete_schema, request.json)
        dm.deleteStudent(data["id"])
        return make_response(jsonify({"success": True}), HTTPStatus.OK)
    except ValidationError as err:
        return make_response(jsonify({"error": err.messages}), HTTPStatus.BAD_REQUEST)

if __name__ == "__main__":
    app.debug = True
    app.run(host='0.0.0.0', port=55001)