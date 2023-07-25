-- Your SQL goes here
CREATE TABLE changelogs (
    artifact_id SERIAL PRIMARY KEY,
    commit_id CHAR(7) NOT NULL,
    commited_at DATE NOT NULL,
    commit_comment VARCHAR(2048) NOT NULL,
    commited_by VARCHAR(255) NOT NULL,
    release_id SERIAL REFERENCES releases(id)
);
