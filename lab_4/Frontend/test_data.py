groups = [
    {
        "id": 1,
        "name":"group 1",
        "leaderId": 1
    },
    {
        "id": 2,
        "name":"group 2",
        "leaderId": 2
    }
]

students = [
    {
        "id": 1,
        "groupId": 1,
        "name": "Holovnia",
        "surname": "Oleksandr"
    },
    {
        "id": 2,
        "groupId": 2,
        "name": "Kirill",
        "surname": "Sidak"
    },
    {
        "id": 3,
        "groupId": 2,
        "name": "Sergey",
        "surname": "Panchenko"
    },
    {
        "id": 4,
        "groupId": 1,
        "name": "nameid4",
        "surname": "surnameid4"
    },
    {
        "id": 5,
        "groupId": 1,
        "name": "nameid5",
        "surname": "surnameid5"
    },
    {
        "id": 6,
        "groupId": 1,
        "name": "nameid6",
        "surname": "surnameid6"
    },
]

schedule = [
    {
        "groupId": 1,
        "day": 1,
        "pair": 1,
        "name": "Infrastructura",
        "description": "programming, algorithms, systems design"
    },
    {
        "groupId": 1,
        "day": 1,
        "pair": 2,
        "name": "Computer Science",
        "description": "programming, algorithms, systems design"
    },
    {
        "groupId": 2,
        "day": 2,
        "pair": 1,
        "name": "Mathematics",
        "description": "numbers, equations, problem solving"
    },
    {
        "groupId": 2,
        "day": 2,
        "pair": 2,
        "name": "Economics",
        "description": "markets, resources, decision making"
    },
    {
        "groupId": 2,
        "day": 2,
        "pair": 3,
        "name": "Physics",
        "description": "matter, energy, natural laws"
    },
]

leaders = [group["leaderId"] for group in groups]