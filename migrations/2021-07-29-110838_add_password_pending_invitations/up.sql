-- Your SQL goes here
ALTER TABLE invitations
ADD COLUMN password_pending BOOLEAN NOT NULL DEFAULT FALSE;