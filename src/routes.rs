use std::time::{SystemTime, UNIX_EPOCH};
use rocket::{get, State};
use rocket::http::Status;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use tokio::time;
use crate::FencedDB;
use crate::store::{Entry};


#[get("/?<from>&<limit>")]
pub(crate) fn index(state: &State<FencedDB>, from: Option<u64>, limit: Option<u64>) -> Result<Template, Status> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

    let entries = Entry::
    get_entries(&mut state.lock().map_err(|_| Status::InternalServerError)?, from.unwrap_or(now), limit.unwrap_or(20))
        .map_err(|_| Status::InternalServerError)?;

    Ok(Template::render("home", context! { entries: entries }))
}
