-- Your SQL goes here
ALTER TABLE PROJECTS
ADD UNIQUE (name);

  ALTER TABLE userskills
    DROP CONSTRAINT fk_userskills_skills,
 ADD CONSTRAINT fk_userskills_skills
    FOREIGN KEY (skill_id)
        REFERENCES skills(id);
 
  ALTER TABLE userskills
    DROP CONSTRAINT fk_userskills_skillscopelevels,
  ADD CONSTRAINT fk_userskills_skillscopelevels
    FOREIGN KEY (skillscopelevel_id)
        REFERENCES skillscopelevels(id);

alter TABLE skills
    DROP CONSTRAINT fk_skills_skillcategories,
    add CONSTRAINT fk_skills_skillcategories
        FOREIGN KEY (skillcategory_id)
        REFERENCES skillcategories(id);

alter TABLE skills
    DROP  CONSTRAINT fk_skills_skillscopes,
    add CONSTRAINT fk_skills_skillscopes
        FOREIGN KEY (skillscope_id)
        REFERENCES skillscopes(id);

alter TABLE projectneedskills
    drop CONSTRAINT fk_projectneedskills_skills,
    add CONSTRAINT fk_projectneedskills_skills
        FOREIGN KEY (skill_id)
        REFERENCES skills(id);

alter TABLE projectneedskills
   drop CONSTRAINT fk_projectneedskills_skillscopelevels,
   add CONSTRAINT fk_projectneedskills_skillscopelevels
        FOREIGN KEY (skillscopelevel_id)
        REFERENCES skillscopelevels(id);
