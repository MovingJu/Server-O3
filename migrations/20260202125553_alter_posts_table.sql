-- Add migration script here
ALTER TABLE posts
ALTER COLUMN user_id DROP DEFAULT
