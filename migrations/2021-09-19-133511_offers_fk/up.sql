delete from offers
where project_id not in (select id from projects);

delete from offers
where user_id not in (select id from users);

ALTER TABLE offers
    ADD CONSTRAINT fk_offers_users
    FOREIGN KEY (user_id) 
    REFERENCES users (id)
    ON DELETE CASCADE;

ALTER TABLE offers
   ADD CONSTRAINT fk_offers_projects
    Foreign key (project_id)
    References projects (id)
    ON DELETE CASCADE;
