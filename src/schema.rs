table! {
	invitations (id) {
		id -> Uuid,
		email -> Varchar,
		password_plain -> Varchar,
		expires_at -> Timestamp,
	}
}

table! {
	users (email) {
		uid -> Uuid,
		email -> Varchar,
		hash -> Varchar,
		created_at -> Timestamp,
	}
}

allow_tables_to_appear_in_same_query!(invitations, users,);
