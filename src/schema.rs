table! {
	invitations (id) {
		id -> Uuid,
		email -> Varchar,
		password_plain -> Varchar,
		first_name -> Varchar,
		last_name -> Varchar,
		expires_at -> Timestamp,
		updated_by -> Varchar,
	}
}

table! {
	projects (id) {
		id -> Uuid,
		available -> Bool,
		name -> Varchar,
		updated_by -> Varchar,
	}
}

table! {
	users (id) {
		id -> Uuid,
		isadmin -> Bool,
		ispro -> Bool,
		available -> Bool,
		email -> Varchar,
		firstname -> Varchar,
		lastname -> Varchar,
		hash -> Varchar,
		inserted_at -> Timestamp,
		updated_by -> Varchar,

	}
}

table! {
	projectskills (id) {
		id -> Uuid,
		project_id -> Uuid,
		skill_id -> Uuid,
		skillscopelevel_id -> Nullable<Uuid>,
		min_years -> Nullable<Float4>,
		max_years -> Nullable<Float4>,
		countofusers -> Integer,
		begin_time -> Timestamp,
		end_time -> Timestamp,
		percentage -> Integer,
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
		index -> Int2,
		percentage -> Nullable<Int2>,
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
	userreservations (id) {
		id -> Uuid,
		user_id -> Uuid,
		description -> Varchar,
		begin_time -> Nullable<Timestamp>,
		end_time -> Nullable<Timestamp>,
		percentage -> Nullable<Int2>,
		updated_by -> Varchar,
	}
}

table! {
	userskills (id) {
		id -> Uuid,
		user_id -> Uuid,
		skill_id -> Uuid,
		skillscopelevel_id -> Uuid,
		years -> Nullable<Float4>,
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
		required_minyears -> Nullable<Float4>,
		required_maxyears -> Nullable<Float4>,
		firstname -> Varchar,
		lastname -> Varchar,
		available -> Bool,
		user_level -> Varchar,
		user_index -> Integer,
		user_years -> Nullable<Float4>,
	}
}

joinable!(projectskills -> projects (project_id));
joinable!(skills -> skillcategories (skillcategory_id));
joinable!(skills -> skillscopes (skillscope_id));
joinable!(skillscopelevels -> skillscopes (skillscope_id));
joinable!(userreservations -> users (user_id));
joinable!(userskills -> users (user_id));

allow_tables_to_appear_in_same_query!(
	invitations,
	projects,
	projectskills,
	skillcategories,
	skills,
	skillscopelevels,
	skillscopes,
	userreservations,
	users,
	userskills,
);
