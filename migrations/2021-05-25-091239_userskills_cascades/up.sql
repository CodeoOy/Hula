ALTER TABLE userskills
    DROP CONSTRAINT fk_userskills_users,
 ADD CONSTRAINT fk_userskills_users
    FOREIGN KEY (user_id)
        REFERENCES users(id)
         ON DELETE CASCADE;
  ALTER TABLE userskills
    DROP CONSTRAINT fk_userskills_skills,
 ADD CONSTRAINT fk_userskills_skills
    FOREIGN KEY (skill_id)
        REFERENCES skills(id)
        ON DELETE CASCADE;
 
  ALTER TABLE userskills
    DROP CONSTRAINT fk_userskills_skillscopelevels,
  ADD CONSTRAINT fk_userskills_skillscopelevels
    FOREIGN KEY (skillscopelevel_id)
        REFERENCES skillscopelevels(id)
        ON DELETE CASCADE;