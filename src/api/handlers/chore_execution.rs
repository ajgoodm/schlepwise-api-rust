use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::api;
use crate::api::handlers::utils::{error_status, host, port};
use crate::api::model::{ChoreExecution, NewChoreExecution, NewChoreExecutionDetails};
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

#[post(
    "/<household_id_>/family_members/<family_member_id_>/chore_executions",
    format = "application/json",
    data = "<new_chore_execution_details>"
)]
pub fn create_chore_execution(
    household_id_: i32,
    family_member_id_: i32,
    new_chore_execution_details: Json<NewChoreExecutionDetails>,
    connection: DbConn,
) -> Result<status::Created<Json<ChoreExecution>>, Status> {
    let new_chore_execution = Json(NewChoreExecution {
        started_at: new_chore_execution_details.started_at,
        finished_at: new_chore_execution_details.finished_at,
        chore_id: new_chore_execution_details.chore_id,
        executed_by_family_member_id: family_member_id_,
    });

    match api::repositories::chore_execution::create_chore_execution(
        new_chore_execution.into_inner(),
        &connection,
    ) {
        Ok(chore_execution) => Ok(chore_execution_created(household_id_, chore_execution)),
        Err(err) => Err(error_status(err)),
    }
}

fn chore_execution_created(
    household_id_: i32,
    chore_execution: ChoreExecution,
) -> status::Created<Json<ChoreExecution>> {
    status::Created(
        format!(
            "{host}:{port}/households/{household_id_}/family_members/{family_member_id_}/chore_executions",
            host = host(),
            port = port(),
            household_id_ = household_id_,
            family_member_id_ = chore_execution.executed_by_family_member_id,
        ),
        Some(Json(chore_execution)),
    )
}
