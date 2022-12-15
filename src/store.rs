use std::str::FromStr;
use std::string::FromUtf8Error;
use std::sync::{LockResult, MutexGuard};
use regex::Regex;
use rocket::serde::Serialize;
use rusqlite::Connection;
use crate::client::{ClientError, Hit, Root};
use thiserror::Error;
use crate::store::EntryKind::Unknown;
use crate::store::StoreError::{ParseError, RegexError};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("...")]
    ParseError,
    #[error("...")]
    RegexError(#[from] regex::Error),
    #[error("...")]
    UTF8Error(#[from] FromUtf8Error),
    #[error("...")]
    SQLError(#[from] rusqlite::Error),

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum EntryKind {
    Story,
    Comment,
    Unknown,
}

impl Default for EntryKind {
    fn default() -> Self {
        Unknown
    }
}

impl ToString for EntryKind {
    fn to_string(&self) -> String {
        match self {
            EntryKind::Story => String::from("story"),
            EntryKind::Comment => String::from("comment"),
            EntryKind::Unknown => String::from("unknown")
        }
    }
}

impl FromStr for EntryKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "story" => Ok(Self::Story),
            "comment" => Ok(Self::Comment),
            _ => Ok(Self::Unknown)
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Entry {
    id: String,
    kind: EntryKind,
    timestamp: u64,
    pdf_link: String,
    story_title: String,
    comment_text: String,
    parent_id: String,
    author: String,
}


impl Entry {
    pub fn from_hit(r: &Hit) -> Result<Entry, StoreError> {
        let pdf_re = Regex::new(r".*(https?://.*pdf)")?;

        let mut entry = Entry {
            id: r.object_id.clone(),
            timestamp: r.created_at_i,
            parent_id: format!("{}", r.parent_id.unwrap_or_default()),
            author: r.author.clone(),
            ..Default::default()
        };

        if let Some(s) = &r.story_title {
            entry.story_title = s.clone();
        }
        if let Some(s) = &r.title {
            entry.story_title = s.clone();
        }

        if let Some(u) = &r.url {
            if let Ok(u) = Entry::extract_url(u) {
                entry.pdf_link = u;
                entry.kind = EntryKind::Story;
            }
        }

        if let Some(comment) = &r.comment_text {
            if let Ok(u) = Entry::extract_url(comment) {
                entry.pdf_link = u;
                entry.kind = EntryKind::Comment;
            }
            entry.comment_text = comment.clone();
        }

        if entry.kind == EntryKind::Unknown {
            return Err(ParseError);
        }


        Ok(entry)
    }

    fn extract_url(s: &str) -> Result<String, StoreError> {
        let re = Regex::new(r".*(https?://.*pdf)")?;
        let caps = re.captures(s).ok_or(ParseError)?;

        Ok(caps.get(1)
            .map_or("", |m| m.as_str())
            .to_string())
    }

    pub fn store_entries(conn: &mut MutexGuard<Connection>, entries: &[Entry]) -> Result<(), StoreError> {
        for e in entries {
            conn.execute("INSERT OR IGNORE INTO contents(id, kind, timestamp, link, story_title, comment_text, parent_id, author) VALUES(?, ?, ?, ?, ?,?, ?, ?)",
                         [&e.id, &e.kind.to_string(), &format!("{}", e.timestamp), &e.pdf_link, &e.story_title, &e.comment_text, &e.parent_id, &e.author])?;
        }

        Ok(())
    }
    pub fn get_entries(conn: &mut MutexGuard<Connection>) -> Result<Vec<Entry>, StoreError> {
        let mut stmt = conn.prepare(
            "SELECT id, kind, timestamp, link, story_title, comment_text, parent_id, author FROM contents ORDER BY timestamp DESC LIMIT 5")?;

        let entries = stmt.query_map([], |row| {
            let kind_str: String = row.get(1)?;


            Ok(Entry {
                id: row.get(0)?,
                kind: EntryKind::from_str(kind_str.as_str()).unwrap(),
                timestamp: row.get(2)?,
                pdf_link: row.get(3)?,
                story_title: row.get(4)?,
                comment_text: row.get(5)?,
                parent_id: row.get(6)?,
                author: row.get(7)?,
            })
        })?.filter_map(|x| x.ok()).collect::<Vec<Entry>>();

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Hit;
    use crate::store::Entry;

    #[test]
    fn test() {
        let h =
            Hit {
                created_at: "2022-12-13T13:05:57.000Z".to_string(),
                title: Some("SEC Complaint Against Sam Bankman-Fried [pdf]".to_string()),
                url: Some("https://www.sec.gov/litigation/complaints/2022/comp-pr2022-219.pdf".to_string()),
                author: "".to_string(),
                points: None,
                story_text: Default::default(),
                comment_text: None,
                num_comments: None,
                story_id: None,
                story_title: None,
                story_url: None,
                parent_id: None,
                created_at_i: 0,
                tags: vec![],
                object_id: "".to_string(),
            };

        assert!(Entry::from_hit(&h).is_ok());
    }
}
