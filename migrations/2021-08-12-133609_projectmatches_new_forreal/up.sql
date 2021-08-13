-- Your SQL goes here
DROP VIEW projectmatches;
DROP VIEW projectskills;

-- projectmatches view
create view projectmatches as
  select 
    row_number() OVER ()::integer AS idx,
    p.id as project_id,
    s.label as skill_label,
    pn.id as pn_id,
    pn.label as pn_label, 
    pn.percentage AS required_load,
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

-- projectskills view
create or replace view projectskills as
  select 
    row_number() OVER ()::integer AS idx,
    p.id as project_id,
    s.label as skill_label,
    pn.id as pn_id,
    lp.index as required_index,
    pns.min_years as required_minyears,
    pns.max_years as required_maxyears
  from 
    projects p,
    projectneeds pn,
    projectneedskills pns
    left join skillscopelevels lp
			on pns.skillscopelevel_id = lp.id,
    skills s
  where 
    pns.projectneed_id = pn.id 
    and pn.project_id = p.id 
    and pns.skill_id = s.id; 