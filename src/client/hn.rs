use html_escape::{decode_html_entities};
use crate::client::types::*;

pub async fn search_by_date(text: &str, page: Option<usize>) -> Result<Root, ClientError> {
    let mut req_url = format!("https://hn.algolia.com/api/v1/search_by_date?query={}", text);
    if let Some(n) = page {
        req_url = format!("{}&page={}", req_url, n);
    }
    let mut result = reqwest::get(req_url)
        .await?
        .json::<Root>()
        .await?;

    for i in result.hits.iter_mut() {
        if let Some(c) = &mut i.comment_text {
            i.comment_text = Some(decode_html_entities(c).parse()?)
        }
    }

    Ok(result)
}

