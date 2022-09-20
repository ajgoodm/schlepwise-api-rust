use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::api;
use crate::api::handlers::utils::{error_status, host, port};
use crate::api::model::{Household, NewHousehold};
use crate::connection::DbConn;

#[get("/")]
pub fn all_households(connection: DbConn) -> Result<Json<Vec<Household>>, Status> {
    api::repositories::household::show_households(&connection)
        .map(Json)
        .map_err(error_status)
}

#[post("/", format = "application/json", data = "<new_household>")]
pub fn create_household(
    new_household: Json<NewHousehold>,
    connection: DbConn,
) -> Result<status::Created<Json<Household>>, Status> {
    api::repositories::household::create_household(new_household.into_inner(), &connection)
        .map(household_created)
        .map_err(error_status)
}

#[get("/<id>")]
pub fn get_household(id: i32, connection: DbConn) -> Result<Json<Household>, Status> {
    api::repositories::household::get_household(id, &connection)
        .map(Json)
        .map_err(error_status)
}

#[put("/<id>", format = "application/json", data = "<household>")]
pub fn update_household(
    id: i32,
    household: Json<Household>,
    connection: DbConn,
) -> Result<Json<Household>, Status> {
    api::repositories::household::update_household(id, household.into_inner(), &connection)
        .map(Json)
        .map_err(error_status)
}

#[delete("/<id>")]
pub fn delete_household(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    api::repositories::household::delete_household(id, &connection)
        .map(|_| status::NoContent)
        .map_err(error_status)
}

fn household_created(household: Household) -> status::Created<Json<Household>> {
    status::Created(
        format!(
            "{host}:{port}/households/{id}",
            host = host(),
            port = port(),
            id = household.id
        )
        .to_string(),
        Some(Json(household)),
    )
}
