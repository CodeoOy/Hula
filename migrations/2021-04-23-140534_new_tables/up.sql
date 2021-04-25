
-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION hula_manage_table(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER hula_set_inserted BEFORE INSERT ON %s
                    FOR EACH ROW EXECUTE PROCEDURE hula_set_inserted()', _tbl);
    EXECUTE format('CREATE TRIGGER hula_set_updated BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE hula_set_updated()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION hula_set_inserted() RETURNS trigger AS $$
BEGIN
    NEW.inserted_at := current_timestamp;
    NEW.updated_at := NEW.inserted_at;
    NEW.inserted_by := NEW.updated_by;
    NEW.updated_count := 0;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION hula_set_updated() RETURNS trigger AS $$
BEGIN
    IF (NEW IS DISTINCT FROM OLD) THEN
        NEW.inserted_by := OLD.inserted_by;
        NEW.inserted_at := OLD.inserted_at;
        NEW.updated_at := current_timestamp;
        NEW.updated_count := OLD.updated_count + 1;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TABLE skillcategories (
  id UUID NOT NULL PRIMARY KEY,
  label VARCHAR(100) NOT NULL,
  parentid UUID NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL
);

CREATE UNIQUE INDEX skillcategories_label ON skillcategories (label);

SELECT hula_manage_table('skillcategories');

CREATE TABLE skillscopes (
  id UUID NOT NULL PRIMARY KEY,
  label VARCHAR(100) NOT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL
);

CREATE UNIQUE INDEX skillscopes_label ON skillscopes (label);

SELECT hula_manage_table('skillscopes');

CREATE TABLE skillscopelevels (
  id UUID NOT NULL PRIMARY KEY,
  label VARCHAR(100) NOT NULL,
  skillscopeid UUID NOT NULL,
  index INT NOT NULL,
  percentage INT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_skillscopelevels_skillscopes
    FOREIGN KEY (skillscopeid)
        REFERENCES skillscopes(id)
);

CREATE UNIQUE INDEX skillscopelevels_index ON skillscopelevels (skillscopeid, index);
CREATE UNIQUE INDEX skillscopelevels_label ON skillscopelevels (skillscopeid, label);

SELECT hula_manage_table('skillscopelevels');

CREATE TABLE skills (
  id UUID NOT NULL PRIMARY KEY,
  label VARCHAR(100) NOT NULL,
  skillcategoryid UUID NOT NULL,
  skillscopeid UUID NOT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_skills_skillcategories
    FOREIGN KEY (skillcategoryid)
        REFERENCES skillcategories(id),
  CONSTRAINT fk_skills_skillscopes
    FOREIGN KEY (skillscopeid)
        REFERENCES skillscopes(id)
);

CREATE UNIQUE INDEX skills_index ON skills (skillcategoryid, label);

SELECT hula_manage_table('skills');

CREATE TABLE userskills (
  id UUID NOT NULL PRIMARY KEY,
  userid UUID NOT NULL,
  skillid UUID NOT NULL,
  skillscopelevelid UUID NULL,
  years REAL NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_userskills_users
    FOREIGN KEY (userid)
        REFERENCES users(uid),
  CONSTRAINT fk_userskills_skills
    FOREIGN KEY (skillid)
        REFERENCES skills(id),
  CONSTRAINT fk_userskills_skillscopelevels
    FOREIGN KEY (skillscopelevelid)
        REFERENCES skillscopelevels(id)
);

CREATE UNIQUE INDEX userskills_skill ON userskills (userid, skillid);

SELECT hula_manage_table('userskills');

CREATE TABLE projectskills (
  id UUID NOT NULL PRIMARY KEY,
  projectid UUID NOT NULL,
  skillid UUID NOT NULL,
  skillscopelevelid UUID NULL,
  minyears REAL NULL,
  maxyears REAL NULL,
  countofusers INT NOT NULL,
  begin_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NULL,
  percentage INT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_projectskills_projects
    FOREIGN KEY (projectid)
        REFERENCES projects(pid),
  CONSTRAINT fk_userskills_skills
    FOREIGN KEY (skillid)
        REFERENCES skills(id),
  CONSTRAINT fk_userskills_skillscopelevels
    FOREIGN KEY (skillscopelevelid)
        REFERENCES skillscopelevels(id)
);

SELECT hula_manage_table('projectskills');

CREATE TABLE userreservations (
  id UUID NOT NULL PRIMARY KEY,
  userid UUID NOT NULL,
  description VARCHAR(1000) NOT NULL,
  begin_time TIMESTAMP NULL,
  end_time TIMESTAMP NULL,
  percentage INT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_projectskills_users
    FOREIGN KEY (userid)
        REFERENCES users(uid)
);

SELECT hula_manage_table('userreservations');


create view matchcandidates as
select 
	ps.id "projectskillid",
	uk.id "userskillid",
	p.name "projectname",
	s.label "skillname",
	lp.label "required_level",
	lp.index "required_index",
	ps.minyears "required_minyears",
	ps.maxyears "required_maxyears",
	u.firstname,
	u.lastname,
	up.label "user_level",
	up.index "user_index",
	uk.years "user_years"
from 
	projectskills ps, 
	projects p, 
	userskills uk, 
	users u, 
	skills s, 
	skillscopelevels lp,
	skillscopelevels up
where ps.skillid = uk.skillid
and ps.projectid = p.pid
and ps.skillid = s.id
and ps.skillscopelevelid = lp.id
and uk.userid = u.uid
and uk.skillscopelevelid = up.id;


