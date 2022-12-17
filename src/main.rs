mod client;
mod store;
mod routes;
mod background;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use envconfig::Envconfig;
use rusqlite::{Connection};
use rocket::routes;
use rocket_dyn_templates::{Template};


const SCHEMA: &str = include_str!("../res/schema.sql");

type FencedDB = Arc<Mutex<Connection>>;


#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "SQLLITE_DB_NAME", default = "db.sqlite")]
    pub db_name: String,
    #[envconfig(from = "SCRAPE_INTERVAL", default = "300")]
    pub scrape_interval: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::init_from_env()?;

    let conn = db_connection(&config)?;


    let _ = background::start_scraping(conn.clone(), Duration::from_secs(config.scrape_interval)).await;

    let _rocket = rocket::build()
        .mount("/", routes![routes::home, routes::single])
        .attach(Template::fairing())
        .manage(conn.clone())
        .launch()
        .await?;

    Ok(())
}

fn db_connection(config: &Config) -> Result<FencedDB, Box<dyn std::error::Error>> {
    let connection = Connection::open(config.db_name.clone())?;
    connection.execute(SCHEMA, [])?;
    Ok(Arc::new(Mutex::new(connection)))
}
