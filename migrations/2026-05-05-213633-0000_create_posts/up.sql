-- Your SQL goes here
CREATE TABLE posts
(
    id    SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body  TEXT    NOT NULL,
    slug  VARCHAR NOT NULL DEFAULT FALSE
)