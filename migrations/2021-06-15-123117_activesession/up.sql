create view activesessions as
select 
	s.id "session_id",
	u.id "user_id",
	u.email "email",
	s.expire_at "expire_at",
	u.isadmin "isadmin"
from 
	users u, 
	sessions s
where u.id = s.user_id;

