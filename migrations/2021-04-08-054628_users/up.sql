-- Your SQL goes here

CREATE TABLE users (
  uid UUID NOT NULL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  hash VARCHAR(122) NOT NULL, --argon hash
  created_at TIMESTAMP NOT NULL
);