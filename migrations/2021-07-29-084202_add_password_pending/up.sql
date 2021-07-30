-- Your SQL goes here
ALTER TABLE users
ADD COLUMN password_pending BOOLEAN NOT NULL DEFAULT FALSE;