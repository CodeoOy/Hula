table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        password_plain -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    projects (pid) {
        pid -> Uuid,
        available -> Bool,
        name -> Varchar,
    }
}

table! {
    users (uid) {
        uid -> Uuid,
        isadmin -> Bool,
        ispro -> Bool,
        available -> Bool,
        email -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    invitations,
    projects,
    users,
);
