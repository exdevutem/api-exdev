-- Add up migration script here
CREATE TABLE club_members (
      uuid TEXT PRIMARY KEY NOT NULL,
      name TEXT NOT NULL,
      birthday TEXT,
      state TEXT CHECK(state IN ("Active", "Unactive", "Graduated", "NoLongerAMember")) DEFAULT "Active" NOT NULL,
      email TEXT,
      github TEXT,
      created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
      updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
  );

CREATE TABLE projects (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    state TEXT DEFAULT "NotStarted" NOT NULL,
      created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
      updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE project_involvement (
    id INTEGER PRIMARY KEY NOT NULL,
    club_member_uuid TEXT NOT NULL,
    project_uuid TEXT NOT NULL,
    FOREIGN KEY(club_member_uuid) REFERENCES club_members(uuid),
    FOREIGN KEY(project_uuid) REFERENCES projects(uuid)
    );
