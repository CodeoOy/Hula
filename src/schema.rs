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
    projectskills (id) {
        id -> Uuid,
        projectid -> Uuid,
        skillid -> Uuid,
        skillscopelevelid -> Nullable<Uuid>,
        minyears -> Nullable<Numeric>,
        maxyears -> Nullable<Numeric>,
        countofusers -> Numeric,
        begin_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        percentage -> Nullable<Int2>,
        inserted_by -> Varchar,
        inserted_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        updated_count -> Int2,
    }
}

table! {
    skillcategories (id) {
        id -> Uuid,
        label -> Varchar,
        parentid -> Nullable<Uuid>,
        inserted_by -> Varchar,
        inserted_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        updated_count -> Int2,
    }
}

table! {
    skills (id) {
        id -> Uuid,
        label -> Varchar,
        skillcategoryid -> Uuid,
        skillscopeid -> Uuid,
        inserted_by -> Varchar,
        inserted_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        updated_count -> Int2,
    }
}

table! {
    skillscopelevels (id) {
        id -> Uuid,
        label -> Varchar,
        skillscopeid -> Uuid,
        index -> Int2,
        percentage -> Nullable<Int2>,
        inserted_by -> Varchar,
        inserted_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        updated_count -> Int2,
    }
}

table! {
    skillscopes (id) {
        id -> Uuid,
        label -> Varchar,
        inserted_by -> Varchar,
        inserted_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        updated_count -> Int2,
    }
}

table! {
    userreservations (id) {
        id -> Uuid,
        userid -> Uuid,
        description -> Varchar,
        begin_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        percentage -> Nullable<Int2>,
        inserted_by -> Varchar,
        inserted_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        updated_count -> Int2,
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

table! {
    userskills (id) {
        id -> Uuid,
        userid -> Uuid,
        skillid -> Uuid,
        skillscopelevelid -> Nullable<Uuid>,
        years -> Nullable<Numeric>,
        inserted_by -> Varchar,
        inserted_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        updated_count -> Int2,
    }
}

joinable!(projectskills -> projects (projectid));
joinable!(skills -> skillcategories (skillcategoryid));
joinable!(skills -> skillscopes (skillscopeid));
joinable!(skillscopelevels -> skillscopes (skillscopeid));
joinable!(userreservations -> users (userid));
joinable!(userskills -> users (userid));

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
