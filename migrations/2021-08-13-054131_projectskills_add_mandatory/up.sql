DROP VIEW projectskills;

create or replace view projectskills as
  select 
    row_number() OVER ()::integer AS idx,
    p.id as project_id,
    s.label as skill_label,
    pn.id as pn_id,
    lp.index as required_index,
    pns.mandatory as is_mandatory,
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