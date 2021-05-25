Create table user_favorites(
id uuid not null primary key,
user_id uuid not null,
project_id uuid not null,
inserted_by VARCHAR(100) NOT NULL,
inserted_at TIMESTAMP NOT NULL,
updated_by VARCHAR(100) NOT NULL,
updated_at TIMESTAMP NOT NULL,
updated_count SMALLINT NOT NULL,

unique(user_id, project_id),

 CONSTRAINT fk_user_favorites_users
Foreign key (user_id)
References users (id),

 CONSTRAINT fk_user_favorites_projects
Foreign key (project_id)
References projects (id)
);
Select hula_manage_table('user_favorites');