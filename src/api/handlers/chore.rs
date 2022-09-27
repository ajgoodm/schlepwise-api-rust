use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::api;
use crate::api::handlers::utils::{error_status, host, port};
use crate::api::model::{Chore, NewChore, NewChoreDetails};
use crate::connection::DbConn;

#[get("/<household_id_>/chores")]
pub fn all_chores(connection: DbConn, household_id_: i32) -> Result<Json<Vec<Chore>>, Status> {
    api::repositories::chore::show_chores(&connection, household_id_)
        .map(Json)
        .map_err(error_status)
}

#[post(
    "/<household_id_>/chores",
    format = "application/json",
    data = "<new_chore_details>"
)]
pub fn create_chore(
    household_id_: i32,
    new_chore_details: Json<NewChoreDetails>,
    connection: DbConn,
) -> Result<status::Created<Json<Chore>>, Status> {
    let new_chore = Json(NewChore {
        name: new_chore_details.name.clone(),
        description: new_chore_details.description.clone(),
        expected_duration_minutes: new_chore_details.expected_duration_minutes,
        created_by_family_member_id: new_chore_details.created_by_family_member_id,
        household_id: household_id_,
    });

    api::repositories::chore::create_chore(household_id_, new_chore.into_inner(), &connection)
        .map(chore_created)
        .map_err(error_status)
}

#[get("/<household_id_>/chores/<id>")]
pub fn get_chore(household_id_: i32, id: i32, connection: DbConn) -> Result<Json<Chore>, Status> {
    api::repositories::chore::get_chore(household_id_, id, &connection)
        .map(Json)
        .map_err(error_status)
}

#[delete("/<household_id_>/chores/<id>")]
pub fn delete_chore(
    household_id_: i32,
    id: i32,
    connection: DbConn,
) -> Result<status::NoContent, Status> {
    api::repositories::chore::delete_chore(household_id_, id, &connection)
        .map(|_| status::NoContent)
        .map_err(error_status)
}

fn chore_created(chore: Chore) -> status::Created<Json<Chore>> {
    status::Created(
        format!(
            "{host}:{port}/households/{household_id_}/chores/{chore_id_}",
            host = host(),
            port = port(),
            household_id_ = chore.household_id,
            chore_id_ = chore.id,
        ),
        Some(Json(chore)),
    )
}
