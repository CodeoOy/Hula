create view projectskills as
 SELECT row_number() OVER ()::integer AS idx,
    p.id AS project_id,
    pn.id AS pn_id,
    pn.label AS pn_label,
    s.label AS skill_label,
    lp.index AS required_index,
    lp.label AS required_label,
    pns.mandatory AS is_mandatory,
    pns.min_years AS required_minyears,
    pns.max_years AS required_maxyears
   FROM projects p,
    projectneeds pn,
    projectneedskills pns
     LEFT JOIN skillscopelevels lp ON pns.skillscopelevel_id = lp.id,
    skills s
  WHERE pns.projectneed_id = pn.id AND pn.project_id = p.id AND pns.skill_id = s.id
