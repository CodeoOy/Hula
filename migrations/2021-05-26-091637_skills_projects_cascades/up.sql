
alter TABLE skills
    DROP CONSTRAINT fk_skills_skillcategories,
    add CONSTRAINT fk_skills_skillcategories
        FOREIGN KEY (skillcategory_id)
        REFERENCES skillcategories(id)
        ON DELETE CASCADE;

alter TABLE skills
    DROP  CONSTRAINT fk_skills_skillscopes,
    add CONSTRAINT fk_skills_skillscopes
        FOREIGN KEY (skillscope_id)
        REFERENCES skillscopes(id)
         ON DELETE CASCADE;
alter TABLE skillscopelevels
    drop CONSTRAINT fk_skillscopelevels_skillscopes,
    add CONSTRAINT fk_skillscopelevels_skillscopes
        FOREIGN KEY (skillscope_id)
        REFERENCES skillscopes(id)
        ON DELETE CASCADE;


alter TABLE projectneedskills
    drop CONSTRAINT fk_projectneedskills_projectneeds,
    add CONSTRAINT fk_projectneedskills_projectneeds
        FOREIGN KEY (projectneed_id)
        REFERENCES projectneeds(id)
        ON DELETE CASCADE;
 
alter TABLE projectneedskills
    drop CONSTRAINT fk_projectneedskills_skills,
    add CONSTRAINT fk_projectneedskills_skills
        FOREIGN KEY (skill_id)
        REFERENCES skills(id)
        ON DELETE CASCADE;
  
alter TABLE projectneedskills 
   drop CONSTRAINT fk_projectneedskills_skillscopelevels,
   add CONSTRAINT fk_projectneedskills_skillscopelevels
        FOREIGN KEY (skillscopelevel_id)
        REFERENCES skillscopelevels(id)
        ON DELETE CASCADE;

alter TABLE projectneeds
    drop CONSTRAINT fk_projectneeds_projects,
    add CONSTRAINT fk_projectneeds_projects
        FOREIGN KEY (project_id)
        REFERENCES projects(id)
        ON DELETE CASCADE;