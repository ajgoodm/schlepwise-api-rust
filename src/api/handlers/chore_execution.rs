use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::api;
use crate::api::handlers::utils::error_status;
use crate::api::model::ChoreExecution;
use crate::connection::DbConn;

#[get("/<household_id_>/chore_executions")]
pub fn all_chore_executions_for_household(
    connection: DbConn,
    household_id_: i32,
) -> Result<Json<Vec<ChoreExecution>>, Status> {
    api::repositories::chore_execution::show_chore_executions_for_household(
        &connection,
        household_id_,
    )
    .map(Json)
    .map_err(error_status)
}

#[get("/<_household_id_>/family_members/<family_member_id_>/chore_executions")]
pub fn all_chore_executions_for_family_member(
    connection: DbConn,
    _household_id_: i32,
    family_member_id_: i32,
) -> Result<Json<Vec<ChoreExecution>>, Status> {
    api::repositories::chore_execution::show_chore_executions_for_family_member(
        &connection,
        family_member_id_,
    )
    .map(Json)
    .map_err(error_status)
}

#[get("/<_household_id_>/chores/<chore_id_>/chore_executions")]
pub fn all_chore_executions_for_chore(
    connection: DbConn,
    _household_id_: i32,
    chore_id_: i32,
) -> Result<Json<Vec<ChoreExecution>>, Status> {
    api::repositories::chore_execution::show_chore_executions_for_chore(&connection, chore_id_)
        .map(Json)
        .map_err(error_status)
}
