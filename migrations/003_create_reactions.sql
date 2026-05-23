CREATE TABLE IF NOT EXISTS reactions (
    id         BIGSERIAL PRIMARY KEY,
    post_id    TEXT        NOT NULL,
    username   TEXT        NOT NULL,
    reaction   VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(post_id, username, reaction)
);
CREATE INDEX IF NOT EXISTS idx_reactions_post_id ON reactions(post_id);
