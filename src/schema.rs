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

table! {
    projectskills (id) {
        id -> Uuid,
        projectid -> Uuid,
        skillid -> Uuid,
        skillscopelevelid -> Nullable<Uuid>,
        minyears -> Nullable<Float4>,
        maxyears -> Nullable<Float4>,
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
        parentid -> Nullable<Uuid>,
        updated_by -> Varchar,
    }
}

table! {
    skills (id) {
        id -> Uuid,
        label -> Varchar,
        skillcategoryid -> Uuid,
        skillscopeid -> Uuid,
        updated_by -> Varchar,
    }
}

table! {
    skillscopelevels (id) {
        id -> Uuid,
        label -> Varchar,
        skillscopeid -> Uuid,
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
        userid -> Uuid,
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
        userid -> Uuid,
        skillid -> Uuid,
        skillscopelevelid -> Nullable<Uuid>,
        years -> Nullable<Numeric>,
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
	    user_level -> Varchar,
	    user_index -> Integer,
	    user_years -> Nullable<Float4>,
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
