-- Your SQL goes here
ALTER TABLE invitations
ADD COLUMN first_name VARCHAR(100) NOT NULL,
ADD COLUMN last_name VARCHAR(100) NOT NULL;
