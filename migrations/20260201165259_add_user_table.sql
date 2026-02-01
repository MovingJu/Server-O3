-- Add migration script here
CREATE TABLE "user" (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL
);