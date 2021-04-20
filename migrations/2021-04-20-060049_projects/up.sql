-- Your SQL goes here

CREATE TABLE projects (
  pid UUID NOT NULL PRIMARY KEY,
  available BOOLEAN,
  name VARCHAR(100) NOT NULL
);