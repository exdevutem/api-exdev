-- Add down migration script here

PRAGMA foreing_keys=off;

DROP TABLE project_involvement;

ALTER TABLE _old_project_involvement RENAME TO  project_involvement;

PRAGMA foreing_keys=on;
