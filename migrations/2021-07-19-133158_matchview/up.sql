-- hula_reserved_date datatype
create type hula_reserved_date as (
    date       date,
    percentage       int4
);

-- hula_get_user_reservation function
create or replace function hula_get_user_reservation(u_id uuid, b_time date, e_time date)
  returns int4 as $$
declare
		reserved int4 := 0;
		highest int4 := 0;
		reserved_dates hula_reserved_date[];
		m hula_reserved_date;
		rec_res   record;
		cur_reservations cursor(u_id uuid)
    for select begin_time, end_time, percentage
        from userreservations
        where user_id = u_id
        order by begin_time;				 
begin
    -- open the cursor
    open cur_reservations(u_id);
	
    loop
      -- fetch row into the film
      fetch cur_reservations into rec_res;
      -- exit when no more row to fetch
      exit when not found;

      -- filter out non-relevant entries
      if rec_res.end_time is not null and rec_res.end_time < b_time then
        continue;
      end if;
      if e_time is not null and rec_res.begin_time is not null and rec_res.begin_time > e_time then
        continue;
      end if;

      -- set reservation changes          
      if rec_res.begin_time is not null then
        reserved_dates := array_append (reserved_dates, (rec_res.begin_time, rec_res.percentage)::hula_reserved_date);
      else
        reserved_dates := array_append (reserved_dates, (to_date('2000', 'YYYY'), rec_res.percentage)::hula_reserved_date);
      end if;
        
      if rec_res.end_time is not null then
        reserved_dates := array_append (reserved_dates, (rec_res.end_time-1, -rec_res.percentage)::hula_reserved_date);
      end if;	
    end loop;
		
    -- order reservation changes          
    reserved_dates = ARRAY(SELECT unnest(reserved_dates) ORDER BY 1);

    foreach m slice 0 in array reserved_dates
    loop
      reserved := reserved + m.percentage;
      if reserved > highest then
        highest = reserved;
      end if;
    end loop;
    
    return highest;
end;
$$ language plpgsql;

-- projectmatches view
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
    u.available as user_available,
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
create view projectskills as
  select 
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

    