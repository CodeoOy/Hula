-- Your SQL goes here
drop view matchcandidates; 

alter table userskills alter column years type float8;
alter table projectneedskills alter column min_years type float8;
alter table projectneedskills alter column max_years type float8;

create view matchcandidates as
 SELECT pns.id AS projectneedskillid,
    uk.id AS userskillid,
    p.name AS projectname,
    s.label AS skillname,
    lp.label AS required_level,
    lp.index AS required_index,
    pns.min_years AS required_minyears,
    pns.max_years AS required_maxyears,
    u.firstname,
    u.lastname,
    u.available,
    up.label AS user_level,
    up.index AS user_index,
    uk.years AS user_years
   FROM projectneeds pn,
    projectneedskills pns,
    projects p,
    userskills uk,
    users u,
    skills s,
    skillscopelevels lp,
    skillscopelevels up
  WHERE pns.skill_id = uk.skill_id AND pns.projectneed_id = pn.id AND pn.project_id = p.id AND pns.skill_id = s.id AND pns.skillscopelevel_id = lp.id AND uk.user_id = u.id AND uk.skillscopelevel_id = up.id
	
	
