db = db.getSiblingDB('schedules_db');

if (!db.getCollectionNames().includes('schedule_collection')) {
    db.createCollection('schedule_collection');
}

db.schedule_collection.insertMany([
    {
        "groupId": 1,
        "day": 1,
        "pair": 1,
        "name": "Infrastructure",
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
    }
]);