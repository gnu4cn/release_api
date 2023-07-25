-- Your SQL goes here
CREATE TYPE size_unit AS ENUM ('kb', 'mb', 'gb');

CREATE TABLE artifacts (
    changelog_id SERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    filesize NUMBER(4, 2) NOT NULL,
    filesize_unit size_unit NOT NULL,
    release_id SERIAL REFERENCES releases(id)
);
