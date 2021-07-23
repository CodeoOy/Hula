-- Your SQL goes here
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
		
	close cur_reservations;

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

