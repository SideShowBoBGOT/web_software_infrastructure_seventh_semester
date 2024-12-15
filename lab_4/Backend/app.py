from flask import Flask, jsonify, request
import connectors

app = Flask(__name__)
DEBUG = False

@app.route("/students/get/", methods=["POST"])
def studentsGet():
    result = []
    # db = connectors.mysql_conn()
    # cur = db.cursor()

    # cur.execute("SELECT * FROM students")
    # result = cur.fetchall()

    # db.close()
    return jsonify({"data": result}, 200)


@app.route("/students/getById/", methods=["POST"])
def studentsGetById():
    """
        Expected request: {
            "studentId": int
        }
    """
    db = connectors.mysql_conn()
    cur = db.cursor()

    id = request.get_json()["studentId"]
    cur.execute(f"SELECT * FROM students WHERE id = {id}")
    result = cur.fetchone()

    db.close()
    return jsonify({"data": result}, 200)


@app.route("/students/change/add/", methods=["POST"])
def studentsAdd():
    """
        Expected request: {
            "groupId": int,
            "name": str,
            "surname": str
        }
    """
    print("Students add")
    db = connectors.mysql_conn()
    cur = db.cursor()

    data = request.get_json()
    cur.execute("INSERT INTO students (groupId, name, surname) " +
                f"VALUES ({data['groupId']}, '{data['name']}', '{data['surname']}')")
    db.commit()

    db.close()
    return jsonify(success=True)


@app.route("/students/change/delete/", methods=["POST"])
def studentsDelete():
    """
        Expected request: {
            "studentId": int,
        }
    """
    db = connectors.mysql_conn()
    cur = db.cursor()

    data = request.json
    cur.execute(f"DELETE FROM students WHERE id = {data['studentId']}")
    db.commit()

    db.close()
    return jsonify(success=True)


@app.route("/students/change/group/", methods=["POST"])
def studentsChangeGroup():
    """
        Expected request: {
            "studentId": int,
            "groupId": int
        }
    """
    db = connectors.mysql_conn()
    cur = db.cursor()

    data = request.json
    cur.execute(f"UPDATE students  SET groupId = {data['groupId']} WHERE id = {data['studentId']}")
    db.commit()

    db.close()
    return jsonify(success=True)

@app.route("/groups/get/", methods=["POST"])
def groupsGet():
    result = []
    # db = connectors.mysql_conn()
    # cur = db.cursor()
    # cur.execute("SELECT * FROM groups")
    # result = cur.fetchall()
    # db.close()
    return jsonify({"data": result}, 200)


""" SCHEDULE """


@app.route("/schedule/get/", methods=["POST"])
def scheduleGet():
    db = connectors.mongo_conn()['schedules_db']
    coll = db["schedule_collection"]

    result = coll.find()
    result_list = []
    for doc in result:
        doc['_id'] = str(doc['_id'])
        result_list.append(doc)
    return jsonify({"data": result_list}, 200)


if __name__ == "__main__":
    app.run(host='0.0.0.0', port=55002)
