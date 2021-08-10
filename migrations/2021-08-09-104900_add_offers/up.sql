-- Your SQL goes here

CREATE TABLE offers (
  id UUID NOT NULL PRIMARY KEY,
  user_id UUID NOT NULL,
  project_id UUID NOT NULL,
  sold BOOLEAN,
  comments VARCHAR(300)
);