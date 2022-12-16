use std::time::{SystemTime, UNIX_EPOCH};
use rocket::{get, State};
use rocket::http::Status;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use crate::FencedDB;
use crate::store::{Entry};


#[get("/?<from>&<limit>")]
pub(crate) fn index(state: &State<FencedDB>, from: Option<u64>, limit: Option<u64>) -> Result<Template, Status> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default().as_secs();

    let db = &mut state.lock().map_err(|_| Status::InternalServerError)?;

    let entries = Entry::get_entries(db, from.unwrap_or(now), limit.unwrap_or(10)).map_err(|_| Status::InternalServerError)?;

    Ok(Template::render("home", context! { entries: entries }))
}
