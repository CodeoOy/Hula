-- Your SQL goes here
ALTER TABLE projectneedskills
RENAME COLUMN projectneedid TO projectneed_id;

ALTER TABLE projectneedskills
RENAME COLUMN skillid TO skill_id;

ALTER TABLE projectneedskills
RENAME COLUMN skillscopelevelid TO skillscopelevel_id;

ALTER TABLE projectneedskills
RENAME COLUMN minyears TO min_years;

ALTER TABLE projectneedskills
RENAME COLUMN maxyears TO max_years;

ALTER TABLE projectneeds
RENAME COLUMN projectid TO project_id;
ALTER TABLE projectneeds
RENAME COLUMN countofusers TO count_of_users;

ALTER TABLE skillcategories
RENAME COLUMN parentid TO parent_id;

ALTER TABLE skills
RENAME COLUMN skillcategoryid TO skillcategory_id;
ALTER TABLE skills
RENAME COLUMN skillscopeid TO skillscope_id;

ALTER TABLE skillscopelevels
RENAME COLUMN skillscopeid TO skillscope_id;

ALTER TABLE userreservations
RENAME COLUMN userid TO user_id;

ALTER TABLE userskills
RENAME COLUMN userid TO user_id;
ALTER TABLE userskills
RENAME COLUMN skillid TO skill_id;
ALTER TABLE userskills
RENAME COLUMN skillscopelevelid TO skillscopelevel_id;