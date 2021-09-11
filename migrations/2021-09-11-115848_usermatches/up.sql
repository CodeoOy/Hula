create view userskilldetails as
 SELECT row_number() OVER ()::integer AS idx,
    u.id AS user_id,
    s.label AS skill_label,
    lp.index AS level_index,
    lp.label AS level_label,
		lp.percentage as level_percentage,
    us.years AS years
   FROM users u,
    userskills us
     LEFT JOIN skillscopelevels lp ON us.skillscopelevel_id = lp.id,
    skills s
  WHERE us.user_id = u.id AND us.skill_id = s.id
  