table! {
	invitations (id) {
		id -> Uuid,
		email -> Varchar,
		password_plain -> Nullable<Varchar>,
		first_name -> Varchar,
		last_name -> Varchar,
		expires_at -> Timestamp,
		password_pending -> Bool,
		updated_by -> Varchar,
	}
}

table! {
	offers (id) {
		id -> Uuid,
		user_id -> Uuid,
		project_id -> Uuid,
		sold -> Bool,
		comments -> Nullable<Varchar>,
		updated_by -> Varchar,
	}
}
 
table! {
	projectneeds (id) {
		id -> Uuid,
		project_id -> Uuid,
		count_of_users -> Int4,
		begin_time -> Date,
		end_time -> Nullable<Date>,
		label -> Nullable<Varchar>,
		percentage -> Nullable<Int4>,
		updated_by -> Varchar,
	}
}

table! {
	projectneedskills (id) {
		id -> Uuid,
		projectneed_id -> Uuid,
		skill_id -> Uuid,
		skillscopelevel_id -> Nullable<Uuid>,
		min_years -> Nullable<Float8>,
		max_years -> Nullable<Float8>,
		updated_by -> Varchar,
		mandatory -> Bool,
	}
}

table! {
	projects (id) {
		id -> Uuid,
		is_hidden-> Bool,
		name -> Varchar,
		description -> Nullable<Varchar>,
		updated_by -> Varchar,
	}
}

table! {
	reset_requests (id) {
		id -> Uuid,
		email -> Varchar,
		expires_at -> Timestamp,
	}
}

table! {
	sessions (id) {
		id -> Uuid,
		user_id -> Uuid,
		expire_at -> Timestamp,
		updated_by -> Varchar,
	}
}

table! {
	skillcategories (id) {
		id -> Uuid,
		label -> Varchar,
		parent_id -> Nullable<Uuid>,
		updated_by -> Varchar,
	}
}

table! {
	skills (id) {
		id -> Uuid,
		label -> Varchar,
		skillcategory_id -> Uuid,
		skillscope_id -> Uuid,
		updated_by -> Varchar,
	}
}

table! {
	skillscopelevels (id) {
		id -> Uuid,
		label -> Varchar,
		skillscope_id -> Uuid,
		index -> Int4,
		percentage -> Nullable<Int4>,
		updated_by -> Varchar,
	}
}

table! {
	skillscopes (id) {
		id -> Uuid,
		label -> Varchar,
		updated_by -> Varchar,
	}
}

table! {
	userfavorites (id) {
		id -> Uuid,
		user_id -> Uuid,
		project_id -> Uuid,
		updated_by -> Varchar,
	}
}

table! {
	userreservations (id) {
		id -> Uuid,
		user_id -> Uuid,
		description -> Varchar,
		begin_time -> Nullable<Date>,
		end_time -> Nullable<Date>,
		percentage -> Nullable<Int4>,
		updated_by -> Varchar,
	}
}

table! {
	users (id) {
		id -> Uuid,
		isadmin -> Bool,
		is_hidden -> Bool,
		email -> Varchar,
		firstname -> Varchar,
		lastname -> Varchar,
		hash -> Varchar,
		inserted_at -> Timestamp,
		updated_by -> Varchar,
		is_employee -> Bool,
		password_pending -> Bool,
		main_upload_id -> Nullable<Uuid>,
	}
}

table! {
	userskills (id) {
		id -> Uuid,
		user_id -> Uuid,
		skill_id -> Uuid,
		skillscopelevel_id -> Nullable<Uuid>,
		years -> Nullable<Double>,
		updated_by -> Varchar,
	}
}

table! {
	useruploads (id) {
		id -> Uuid,
		user_id -> Uuid,
		filename -> Varchar,
		updated_by -> Varchar,
	}
}

table! {
	matchcandidates (projectneedskillid, userskillid) {
		projectneedskillid -> Uuid,
		userskillid -> Uuid,
		projectname -> Varchar,
		skillname -> Varchar,
		required_level -> Varchar,
		required_index -> Integer,
		required_minyears -> Nullable<Double>,
		required_maxyears -> Nullable<Double>,
		firstname -> Varchar,
		lastname -> Varchar,
		is_hidden -> Bool,
		user_level -> Varchar,
		user_index -> Integer,
		user_years -> Nullable<Double>,
	}
}

table! {
	activesessions (session_id) {
		session_id -> Uuid,
		user_id -> Uuid,
		email -> Varchar,
		expire_at -> Timestamp,
		isadmin -> Bool,
	}
}

table! {
	projectmatches(idx) {
		idx -> Int4,
		project_id -> Uuid,
		project_name -> Varchar,
		skill_label -> Varchar,
		skill_mandatory -> Bool,
		pn_id -> Uuid,
		pn_label -> Varchar,
		required_load -> Nullable<Int4>,
		required_index -> Nullable<Integer>,
		required_minyears -> Nullable<Double>,
		required_maxyears -> Nullable<Double>,
		user_id -> Uuid,
		user_first_name -> Varchar,
		user_last_name -> Varchar,
		user_is_hidden -> Bool,
		user_load -> Int4,
		user_index -> Nullable<Int4>,
		user_years -> Nullable<Double>,
	}
}

table! {
	projectskills(idx){
		idx -> Int4,
		project_id -> Uuid,
		pn_id -> Uuid,
		pn_label -> Nullable<Varchar>,
		skill_label -> Varchar,
		is_mandatory -> Bool,
		required_index -> Nullable<Integer>,
		required_label -> Nullable<Varchar>,
		required_minyears -> Nullable<Double>,
		required_maxyears -> Nullable<Double>,
	}
}

joinable!(projectneeds -> projects (project_id));
joinable!(projectneedskills -> projectneeds (projectneed_id));
joinable!(projectneedskills -> skills (skill_id));
joinable!(projectneedskills -> skillscopelevels (skillscopelevel_id));
joinable!(sessions -> users (user_id));
joinable!(skills -> skillcategories (skillcategory_id));
joinable!(skills -> skillscopes (skillscope_id));
joinable!(skillscopelevels -> skillscopes (skillscope_id));
joinable!(userfavorites -> projects (project_id));
joinable!(userfavorites -> users (user_id));
joinable!(userreservations -> users (user_id));
joinable!(userskills -> skills (skill_id));
joinable!(userskills -> skillscopelevels (skillscopelevel_id));
joinable!(userskills -> users (user_id));
joinable!(useruploads -> users (user_id));

allow_tables_to_appear_in_same_query!(
	invitations,
	projectneeds,
	projectneedskills,
	projects,
	sessions,
	skillcategories,
	skills,
	skillscopelevels,
	skillscopes,
	userfavorites,
	userreservations,
	users,
	userskills,
	useruploads
);
