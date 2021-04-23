-- Your SQL goes here

CREATE TABLE users (
  uid UUID NOT NULL PRIMARY KEY,
  isadmin BOOLEAN NOT NULL,
  ispro BOOLEAN NOT NULL,
  available BOOLEAN NOT NULL,
  email VARCHAR(100) NOT NULL,
  firstname VARCHAR(100) NOT NULL,
  lastname VARCHAR(100) NOT NULL,
  hash VARCHAR(122) NOT NULL, --argon hash
  created_at TIMESTAMP NOT NULL
);