create or replace function hula_get_user_project_favorite(u_id uuid, p_id uuid)
  returns bool as $$
declare
		favorites int4 := 0;
begin
		select count(*)
		into favorites
		from userfavorites
		where user_id = u_id
		and project_id = p_id;
		
		if favorites > 0 then
			return true;
		end if;
		
		return false;
end;
$$ language plpgsql;

drop view projectmatches;

create view projectmatches as
 SELECT row_number() OVER ()::integer AS idx,
    p.id AS project_id,
    p.name AS project_name,
    s.label AS skill_label,
    pn.id AS pn_id,
    pn.label AS pn_label,
    pn.percentage AS required_load,
    lp.index AS required_index,
    pns.mandatory AS skill_mandatory,
    pns.min_years AS required_minyears,
    pns.max_years AS required_maxyears,
    u.id AS user_id,
    u.firstname AS user_first_name,
    u.lastname AS user_last_name,
    u.is_hidden AS user_is_hidden,
    hula_get_user_reservation(u.id, pn.begin_time, pn.end_time) AS user_load,
    hula_get_user_project_favorite(u.id, p.id) AS user_favorite,
		up.index AS user_index,
    uk.years AS user_years
   FROM projects p,
    projectneeds pn,
    projectneedskills pns
     LEFT JOIN skillscopelevels lp ON pns.skillscopelevel_id = lp.id,
    userskills uk
     LEFT JOIN skillscopelevels up ON uk.skillscopelevel_id = up.id,
    users u,
    skills s
  WHERE pns.skill_id = uk.skill_id AND pns.projectneed_id = pn.id AND pn.project_id = p.id AND pns.skill_id = s.id AND uk.user_id = u.id;
