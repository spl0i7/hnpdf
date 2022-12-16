use std::error::Error;
use std::time;
use crate::{client, FencedDB};
use crate::store::Entry;

pub(crate) async fn fetch_pdfs(conn: &mut FencedDB) -> Result<(), Box<dyn Error + '_>> {
    let mut hits = Vec::new();

    for i in 0..10 {
        let root = client::search_by_date(".pdf", Some(i)).await?;

        hits.append(&mut root
            .hits.into_iter()
            .filter_map(|x| Entry::from_hit(&x).ok())
            .collect::<Vec<Entry>>());
    }

    {
        let mut conn = conn.lock()?;
        Entry::store_entries(&mut conn, &hits)?;
    }

    Ok(())
}

pub(crate) async fn start_scraping(mut db: FencedDB, interval: time::Duration) {
    tokio::spawn(async move {
        loop {
            if let Err(e) = fetch_pdfs(&mut db).await {
                println!("{e}");
            }
            tokio::time::sleep(interval).await;
        }
    });
}