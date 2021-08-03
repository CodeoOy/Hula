-- Your SQL goes here

CREATE TABLE reset_requests (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  expires_at TIMESTAMP NOT NULL
);