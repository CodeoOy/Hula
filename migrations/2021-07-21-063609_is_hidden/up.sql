DROP VIEW projectmatches;
DROP VIEW matchcandidates;

ALTER TABLE users
RENAME COLUMN available to is_hidden;

ALTER TABLE users
DROP COLUMN ispro;

ALTER TABLE projects
RENAME COLUMN available to is_hidden;

create view projectmatches as
  select 
    p.id as project_id,
    s.label as skill_label,
    pn.id as pn_id,
    lp.index as required_index,
    pns.min_years as required_minyears,
    pns.max_years as required_maxyears,
    u.id as user_id,
    u.firstname as user_first_name,
    u.lastname as user_last_name,
    u.is_hidden as user_is_hidden,
    "hula_get_user_reservation"(u.id, pn.begin_time, pn.end_time) as user_load,
    up.index as user_index,
    uk.years as user_years
  from 
    projects p,
    projectneeds pn,
    projectneedskills pns
    left join skillscopelevels lp
      on pns.skillscopelevel_id = lp.id,
    userskills uk
		left join skillscopelevels up
			on uk.skillscopelevel_id = up.id,
    users u,
    skills s
  where 
    pns.skill_id = uk.skill_id 
    and pns.projectneed_id = pn.id 
    and pn.project_id = p.id 
    and pns.skill_id = s.id 
    and uk.user_id = u.id;


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
    u.is_hidden,
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
  WHERE pns.skill_id = uk.skill_id AND pns.projectneed_id = pn.id AND pn.project_id = p.id AND pns.skill_id = s.id AND pns.skillscopelevel_id = lp.id AND uk.user_id = u.id AND uk.skillscopelevel_id = up.id;
