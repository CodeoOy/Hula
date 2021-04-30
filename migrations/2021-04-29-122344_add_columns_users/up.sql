-- Your SQL goes here
ALTER TABLE users
ADD COLUMN inserted_by VARCHAR(100) NOT NULL default 'not set',
ADD COLUMN inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
ADD COLUMN updated_by VARCHAR(100) NOT NULL DEFAULT NOW(),
ADD COLUMN updated_count SMALLINT NOT NULL default 0;
