use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::api;
use crate::api::handlers::utils::{error_status, host, port};
use crate::api::model::{FamilyMember, NewFamilyMember, NewFamilyMemberDetails};
use crate::connection::DbConn;

#[get("/<household_id_>/family_members")]
pub fn all_family_members(
    connection: DbConn,
    household_id_: i32,
) -> Result<Json<Vec<FamilyMember>>, Status> {
    api::repositories::family_member::show_family_members(&connection, household_id_)
        .map(Json)
        .map_err(error_status)
}

#[post(
    "/<household_id_>/family_members",
    format = "application/json",
    data = "<new_family_member_details>"
)]
pub fn create_family_member(
    household_id_: i32,
    new_family_member_details: Json<NewFamilyMemberDetails>,
    connection: DbConn,
) -> Result<status::Created<Json<FamilyMember>>, Status> {
    let new_family_member = Json(NewFamilyMember {
        first_name: new_family_member_details.first_name.clone(),
        last_name: new_family_member_details.last_name.clone(),
        household_id: household_id_,
    });

    api::repositories::family_member::create_family_member(
        household_id_,
        new_family_member.into_inner(),
        &connection,
    )
    .map(family_member_created)
    .map_err(error_status)
}

#[get("/<household_id_>/family_members/<id>")]
pub fn get_family_member(
    household_id_: i32,
    id: i32,
    connection: DbConn,
) -> Result<Json<FamilyMember>, Status> {
    api::repositories::family_member::get_family_member(household_id_, id, &connection)
        .map(Json)
        .map_err(error_status)
}

#[delete("/<household_id_>/family_members/<id>")]
pub fn delete_family_member(
    household_id_: i32,
    id: i32,
    connection: DbConn,
) -> Result<status::NoContent, Status> {
    api::repositories::family_member::delete_family_member(household_id_, id, &connection)
        .map(|_| status::NoContent)
        .map_err(error_status)
}

fn family_member_created(family_member: FamilyMember) -> status::Created<Json<FamilyMember>> {
    status::Created(
        format!(
            "{host}:{port}/households/{household_id_}/family_members/{family_member_id_}",
            host = host(),
            port = port(),
            household_id_ = family_member.household_id,
            family_member_id_ = family_member.id,
        ),
        Some(Json(family_member)),
    )
}
