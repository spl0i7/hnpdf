use std::cmp::{min};
use std::str::FromStr;
use std::string::FromUtf8Error;
use std::sync::MutexGuard;
use regex::Regex;
use rocket::serde::Serialize;
use rusqlite::{Connection, Statement};
use crate::client::{Hit};
use thiserror::Error;
use crate::store::ItemKind::Unknown;
use crate::store::StoreError::{ParseError};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("...")]
    NotFound,
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
pub enum ItemKind {
    Story,
    Comment,
    Unknown,
}

impl Default for ItemKind {
    fn default() -> Self {
        Unknown
    }
}

impl ToString for ItemKind {
    fn to_string(&self) -> String {
        match self {
            ItemKind::Story => String::from("story"),
            ItemKind::Comment => String::from("comment"),
            ItemKind::Unknown => String::from("unknown")
        }
    }
}

impl FromStr for ItemKind {
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
pub struct Item {
    id: String,
    kind: ItemKind,
    timestamp: u64,
    pdf_link: String,
    story_title: String,
    comment_text: String,
    parent_id: String,
    author: String,
}


impl Item {
    pub fn from_hit(r: &Hit) -> Result<Item, StoreError> {
        let mut entry = Item {
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
            if let Ok(u) = Item::extract_url(u) {
                entry.pdf_link = u;
                entry.kind = ItemKind::Story;
            }
        }

        if let Some(comment) = &r.comment_text {
            if let Ok(u) = Item::extract_url(comment) {
                entry.pdf_link = u;
                entry.kind = ItemKind::Comment;
            }
            entry.comment_text = comment.clone();
        }

        if entry.kind == Unknown {
            return Err(ParseError);
        }


        Ok(entry)
    }

    fn extract_url(s: &str) -> Result<String, StoreError> {
        let re = Regex::new(r".*(https?://.*\.pdf)")?;
        let caps = re.captures(s).ok_or(ParseError)?;

        Ok(caps.get(1)
            .map_or("", |m| m.as_str())
            .to_string())
    }

    pub fn store_entries(conn: &mut MutexGuard<Connection>, entries: &[Item]) -> Result<(), StoreError> {
        for e in entries {
            conn.execute("INSERT OR IGNORE INTO contents(id, kind, timestamp, link, story_title, comment_text, parent_id, author) VALUES(?, ?, ?, ?, ?,?, ?, ?)",
                         [&e.id, &e.kind.to_string(), &format!("{}", e.timestamp), &e.pdf_link, &e.story_title, &e.comment_text, &e.parent_id, &e.author])?;
        }

        Ok(())
    }
    pub fn get_list(conn: &mut MutexGuard<Connection>, from: u64, limit: u64) -> Result<Vec<Item>, StoreError> {
        let stmt = conn.prepare(
            "SELECT id, kind, timestamp, link, story_title, comment_text, parent_id, author FROM contents WHERE timestamp < ? ORDER BY timestamp DESC LIMIT ?")?;
        Item::scan_rows(stmt, [from, min(limit, 20)])
    }
    pub fn get_one(conn: &mut MutexGuard<Connection>, id: &str) -> Result<Item, StoreError> {
        let stmt = conn.prepare(
            "SELECT id, kind, timestamp, link, story_title, comment_text, parent_id, author FROM contents WHERE id = ?")?;
        Item::scan_rows(stmt, [id])?.into_iter().next().ok_or(StoreError::NotFound)
    }
    pub fn search(conn: &mut MutexGuard<Connection>, text: &str) -> Result<Vec<Item>, StoreError> {
        let stmt = conn.prepare(
            "SELECT id, kind, timestamp, link, story_title, comment_text, parent_id, author FROM contents WHERE id in (SELECT id FROM content_search WHERE content_search MATCH ? ORDER BY rank LIMIT 50)")?;
        Item::scan_rows(stmt, [text])
    }
    fn scan_rows<P: rusqlite::Params>(mut stmt: Statement, p: P) -> Result<Vec<Item>, StoreError> {
        let entries = stmt.query_map(p, |row| {
            let kind_str: String = row.get(1)?;
            Ok(Item {
                id: row.get(0)?,
                kind: ItemKind::from_str(kind_str.as_str()).unwrap(),
                timestamp: row.get(2)?,
                pdf_link: row.get(3)?,
                story_title: row.get(4)?,
                comment_text: row.get(5)?,
                parent_id: row.get(6)?,
                author: row.get(7)?,
            })
        })?.filter_map(|x| x.ok()).collect::<Vec<Item>>();

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use core::default::Default;
    use crate::client::Hit;
    use crate::store::Item;

    #[test]
    fn test() {
        let h =
            Hit {
                created_at: "2022-12-13T13:05:57.000Z".to_string(),
                title: Some("SEC Complaint Against Sam Bankman-Fried [pdf]".to_string()),
                url: Some("https://www.sec.gov/litigation/complaints/2022/comp-pr2022-219.pdf".to_string()),
                ..Default::default()
            };

        assert!(Item::from_hit(&h).is_ok());
    }
}
