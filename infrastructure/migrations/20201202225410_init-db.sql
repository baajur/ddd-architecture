-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    id       UUID                   PRIMARY KEY,
    nickname CHARACTER VARYING(255) NOT NULL CONSTRAINT users_nickname_unique UNIQUE
);
