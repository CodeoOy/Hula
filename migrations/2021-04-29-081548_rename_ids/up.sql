-- Your SQL goes here
ALTER TABLE users
RENAME COLUMN uid TO id;
ALTER TABLE projects 
RENAME COLUMN pid TO id;