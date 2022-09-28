use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::api;
use crate::api::handlers::utils::error_status;
use crate::api::model::ChoreExecution;
use crate::connection::DbConn;

#[get("/<household_id_>/chore_executions")]
pub fn all_chores_for_household(
    connection: DbConn,
    household_id_: i32,
) -> Result<Json<Vec<ChoreExecution>>, Status> {
    api::repositories::chore_execution::show_chore_executions(&connection, household_id_, None)
        .map(Json)
        .map_err(error_status)
}

#[get("/<household_id_>/family_members/<family_member_id_>/chore_executions")]
pub fn all_chores_for_family_member(
    connection: DbConn,
    household_id_: i32,
    family_member_id_: i32,
) -> Result<Json<Vec<ChoreExecution>>, Status> {
    api::repositories::chore_execution::show_chore_executions(
        &connection,
        household_id_,
        Some(family_member_id_),
    )
    .map(Json)
    .map_err(error_status)
}
