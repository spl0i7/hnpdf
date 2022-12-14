use serde::{Serialize, Deserialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub hits: Vec<Hit>,
    pub nb_hits: i64,
    pub page: i64,
    pub nb_pages: i64,
    pub hits_per_page: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub author: String,
    pub points: Option<i64>,
    #[serde(rename = "story_text")]
    pub story_text: Value,
    #[serde(rename = "comment_text")]
    pub comment_text: Option<String>,
    #[serde(rename = "num_comments")]
    pub num_comments: Option<i64>,
    #[serde(rename = "story_id")]
    pub story_id: Option<i64>,
    #[serde(rename = "story_title")]
    pub story_title: Option<String>,
    #[serde(rename = "story_url")]
    pub story_url: Option<String>,
    #[serde(rename = "parent_id")]
    pub parent_id: Option<i64>,
    #[serde(rename = "created_at_i")]
    pub created_at_i: u64,
    #[serde(rename = "_tags")]
    pub tags: Vec<String>,
    #[serde(rename = "objectID")]
    pub object_id: String,
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("...")]
    HttpError(#[from] reqwest::Error),

    #[error("...")]
    Infallible(#[from] std::convert::Infallible),
}