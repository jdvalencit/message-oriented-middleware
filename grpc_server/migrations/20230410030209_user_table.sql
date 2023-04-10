-- Add migration script here
CREATE TABLE users (
    id            UUID PRIMARY KEY,
    username      VARCHAR(45) NOT NULL UNIQUE,
    password      VARCHAR(80) NOT NULL
);
