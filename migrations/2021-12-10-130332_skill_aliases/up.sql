-- Your SQL goes here
ALTER TABLE skills
ADD COLUMN aliases TEXT[] NOT NULL DEFAULT '{}';