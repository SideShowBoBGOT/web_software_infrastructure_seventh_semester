#!/bin/bash
set -e
mongod --fork --logpath /var/log/mongod.log --bind_ip_all
mongoimport --host localhost --db schedules_db --collection schedule_collection --type json --file /init.json --jsonArray
mongod --shutdown
mongod --bind_ip_all