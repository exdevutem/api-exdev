-- Add up migration script here

PRAGMA foreing_keys=off;

ALTER TABLE project_involvement RENAME TO _old_project_involvement;

CREATE TABLE project_involvement (
    id INTEGER PRIMARY KEY NOT NULL,
    club_member_uuid TEXT NOT NULL,
    project_uuid TEXT NOT NULL,
    CONSTRAINT fk_member_uuid FOREIGN KEY(club_member_uuid) REFERENCES club_members(uuid) ON DELETE CASCADE,
    CONSTRAINT fk_project_uuid FOREIGN KEY(project_uuid) REFERENCES projects(uuid) ON DELETE CASCADE
);

INSERT INTO project_involvement SELECT * FROM _old_project_involvement;

PRAGMA foreing_keys=on;
