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

CREATE VIRTUAL TABLE IF NOT EXISTS content_search USING fts5(id, link, story_title, comment_text, author);

CREATE TRIGGER IF NOT EXISTS content_search_trigger AFTER INSERT
    ON contents
BEGIN
    INSERT INTO content_search(id, link, story_title, comment_text, author) VALUES (NEW.id, NEW.link, NEW.story_title, NEW.comment_text, NEW.author);
END;

INSERT INTO content_search(id, link, story_title, comment_text, author) SELECT id, link, story_title, comment_text, author from contents;