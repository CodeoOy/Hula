ALTER TABLE userfavorites
    DROP CONSTRAINT fk_user_favorites_users,
    ADD CONSTRAINT fk_user_favorites_users
    FOREIGN KEY (user_id) 
    REFERENCES users (id)
    ON DELETE CASCADE;

ALTER TABLE userfavorites
    DROP CONSTRAINT fk_user_favorites_projects,
   ADD CONSTRAINT fk_user_favorites_projects
    Foreign key (project_id)
    References projects (id)
    ON DELETE CASCADE;
