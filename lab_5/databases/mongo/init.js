db = db.getSiblingDB('schedules_db');

if (!db.getCollectionNames().includes('schedule_collection')) {
    db.createCollection('schedule_collection');
}

db.schedule_collection.insertMany([
    {
        "id": 0,
        "name": "IP-11",
    },
    {
        "id": 1,
        "name": "IP-12"
    },
    {
        "id": 2,
        "name": "IP-13"
    },
    {
        "id": 3,
        "name": "IP-15"
    },
]);