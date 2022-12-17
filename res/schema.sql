CREATE TABLE IF NOT EXISTS contents  (
    id TEXT NOT NULL PRIMARY KEY,
    kind TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    link TEXT NOT NULL,
    story_title TEXT,
    comment_text TEXT,
    parent_id TEXT NOT NULL,
    author TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS timestamp_idx ON contents (timestamp);