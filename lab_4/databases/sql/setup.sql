CREATE DATABASE IF NOT EXISTS lab4DB;
USE lab4DB;

CREATE TABLE groups (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    leaderId INT
);

CREATE TABLE students (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    surname VARCHAR(255),
    groupId INT,
    FOREIGN KEY (groupId) REFERENCES groups(id)
);

ALTER TABLE groups
ADD FOREIGN KEY (leaderId) REFERENCES students(id);


INSERT INTO groups (id, name, leaderId)
VALUES (1, 'group 1', NULL),
       (2, 'group 2', NULL);


INSERT INTO students (id, name, surname, groupId)
VALUES (1, 'Holovnia', 'Oleksandr', 1),
       (2, 'Kirill', 'Sidak', 2),
       (3, 'Serhii', 'Panchenko', 2),
       (4, 'nameid4', 'surnameid4', 1),
       (5, 'nameid5', 'surnameid5', 1),
       (6, 'nameid6', 'surnameid6', 1);

UPDATE groups 
SET leaderId = 1 WHERE
id = 1;

UPDATE groups 
SET leaderId = 2 WHERE
id = 2;
