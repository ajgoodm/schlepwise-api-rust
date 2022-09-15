use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::api;
use crate::api::model::FamilyMember;
use crate::api::handlers::utils::error_status;

#[get("/<household_id_>/family_members")]
pub fn all_family_members(connection: DbConn, household_id_: i32) -> Result<Json<Vec<FamilyMember>>, Status> {
    api::repositories::family_member::show_family_members(&connection, household_id_)
        .map(|family_member| Json(family_member))
        .map_err(|error| error_status(error))
}