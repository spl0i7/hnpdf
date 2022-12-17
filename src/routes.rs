use std::time::{SystemTime, UNIX_EPOCH};
use rocket::{get, State};
use rocket::http::Status;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use crate::FencedDB;
use crate::store::{Item, StoreError};


#[get("/?<from>&<limit>")]
pub(crate) fn home(state: &State<FencedDB>, from: Option<u64>, limit: Option<u64>) -> Result<Template, Status> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default().as_secs();

    let db = &mut state.lock().map_err(|_| Status::InternalServerError)?;

    let entries = Item::get_list(db, from.unwrap_or(now), limit.unwrap_or(10)).map_err(match_store_err)?;

    Ok(Template::render("home", context! { entries: entries }))
}

#[get("/item/<id>")]
pub(crate) fn single(state: &State<FencedDB>, id: &str) -> Result<Template, Status> {
    let db = &mut state.lock().map_err(|_| Status::InternalServerError)?;

    let entries = Item::get_one(db, id).map_err(match_store_err)?;

    Ok(Template::render("single", context! { entry: entries }))
}

fn match_store_err(e: StoreError) -> Status {
    match e {
        StoreError::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}