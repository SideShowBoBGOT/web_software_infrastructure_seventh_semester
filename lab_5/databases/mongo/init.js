if (!process.env.MONGO_INITDB_DATABASE) {
    throw new Error("Environment variable MONGO_INITDB_DATABASE is not defined.");
}

if (!process.env.MONGO_COLLECTION) {
    throw new Error("Environment variable MONGO_COLLECTION is not defined.");
}

const dbName = process.env.MONGO_INITDB_DATABASE;
const collectionName = process.env.MONGO_COLLECTION;

db = db.getSiblingDB(dbName);

if (!db.getCollectionNames().includes(collectionName)) {
    db.createCollection(collectionName);
}

db[collectionName].insertMany([
    { id: 0, name: "ІП-11" },
    { id: 1, name: "ІП-12" },
    { id: 2, name: "ІП-13" },
    { id: 3, name: "ІП-15" },
]);
