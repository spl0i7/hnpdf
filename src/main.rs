mod client;
mod store;

use rocket::{get, State};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, LockResult, Mutex};
use std::time::Duration;
use envconfig::Envconfig;
use rusqlite::{Connection};
use tokio::time::sleep;
use crate::store::Entry;
use rocket::routes;
use rocket_dyn_templates::{Template, context};

const SCHEMA: &str = include_str!("../res/schema.sql");

type FencedDB = Arc<Mutex<Connection>>;


#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "SQLLITE_DB_NAME", default = "db.sqlite")]
    pub db_name: String,
}


#[get("/")]
fn hello(s: &State<FencedDB>) -> Template {
    let entries = Entry::get_entries(&mut s.lock().unwrap()).unwrap();

    Template::render("home", context! { entries: entries })
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::init_from_env()?;

    let mut conn = db_connection(config).unwrap();

    let mut db = conn.clone();

    tokio::spawn(async move {
        loop {
            println!("starting fetch cycle");
            if let Err(e) = fetch_pdfs(&mut db).await {
                println!("{}", e);
            }
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });



    let _rocket = rocket::build()
        .mount("/", routes![hello])
        .attach(Template::fairing())
        .manage(conn.clone())
        .launch()
        .await?;

    Ok(())

}

fn db_connection(config: Config) -> Result<FencedDB, Box<dyn std::error::Error>> {
    let connection = Connection::open(config.db_name)?;
    connection.execute(SCHEMA, [])?;
    Ok(Arc::new(Mutex::new(connection)))
}

async fn fetch_pdfs(conn: &mut FencedDB) -> Result<(), Box<dyn Error + '_>> {
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
        let _ = Entry::store_entries(&mut conn, &hits);
    }

    Ok(())
}
