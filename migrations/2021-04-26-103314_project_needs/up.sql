-- Your SQL goes here
drop table projectskills CASCADE;

CREATE TABLE projectneeds (
  id UUID NOT NULL PRIMARY KEY,
  projectid UUID NOT NULL,
  countofusers INT NOT NULL,
  begin_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NULL,
  percentage INT NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_projectneeds_projects
    FOREIGN KEY (projectid)
        REFERENCES projects(pid)
);

CREATE TABLE projectneedskills (
  id UUID NOT NULL PRIMARY KEY,
  projectneedid UUID NOT NULL,
  skillid UUID NOT NULL,
  skillscopelevelid UUID NULL,
  minyears REAL NULL,
  maxyears REAL NULL,
  inserted_by VARCHAR(100) NOT NULL,
  inserted_at TIMESTAMP NOT NULL,
  updated_by VARCHAR(100) NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  updated_count SMALLINT NOT NULL,
  CONSTRAINT fk_projectneedskills_projectneeds
    FOREIGN KEY (projectneedid)
        REFERENCES projectneeds(id),
  CONSTRAINT fk_projectneedskills_skills
    FOREIGN KEY (skillid)
        REFERENCES skills(id),
  CONSTRAINT fk_projectneedskills_skillscopelevels
    FOREIGN KEY (skillscopelevelid)
        REFERENCES skillscopelevels(id)
);

CREATE UNIQUE INDEX projectneedskills_skill ON projectneedskills (projectneedid, skillid);
 
drop index skills_index;
CREATE UNIQUE INDEX skills_index ON skills (label);

SELECT hula_manage_table('projectneeds');
SELECT hula_manage_table('projectneedskills');

create view matchcandidates as
select 
	pns.id "projectneedskillid",
	uk.id "userskillid",
	p.name "projectname",
	s.label "skillname",
	lp.label "required_level",
	lp.index "required_index",
	pns.minyears "required_minyears",
	pns.maxyears "required_maxyears",
	u.firstname,
	u.lastname,
	up.label "user_level",
	up.index "user_index",
	uk.years "user_years"
from 
	projectneeds pn, 
	projectneedskills pns, 
	projects p, 
	userskills uk, 
	users u, 
	skills s, 
	skillscopelevels lp,
	skillscopelevels up
where pns.skillid = uk.skillid
and pns.projectneedid = pn.id
and pn.projectid = p.pid
and pns.skillid = s.id
and pns.skillscopelevelid = lp.id
and uk.userid = u.uid
and uk.skillscopelevelid = up.id;

