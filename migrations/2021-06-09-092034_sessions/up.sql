CREATE TABLE sessions (
    id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL,
    expire_at TIMESTAMP NOT NULL,
    inserted_by VARCHAR(100) NOT NULL,
    inserted_at TIMESTAMP NOT NULL,
    updated_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_sessions_users
    FOREIGN KEY (user_id)
        REFERENCES users(id)
 ON DELETE CASCADE

);
Select hula_manage_table('sessions');
