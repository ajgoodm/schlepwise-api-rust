use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::api;
use crate::api::model::{FamilyMember, FirstNameLastName, NewFamilyMember};
use crate::api::handlers::utils::{error_status, host, port};

#[get("/<household_id_>/family_members")]
pub fn all_family_members(connection: DbConn, household_id_: i32) -> Result<Json<Vec<FamilyMember>>, Status> {
    api::repositories::family_member::show_family_members(&connection, household_id_)
        .map(|family_member| Json(family_member))
        .map_err(|error| error_status(error))
}

#[post("/<household_id_>/family_members", format ="application/json", data = "<first_name_last_name>")]
pub fn create_family_member(
    household_id_: i32,
    first_name_last_name: Json<FirstNameLastName>,
    connection: DbConn,
) -> Result<status::Created<Json<FamilyMember>>, Status> {
    let new_family_member = Json(NewFamilyMember {
        first_name: first_name_last_name.first_name.clone(),
        last_name: first_name_last_name.last_name.clone(),
        household_id: household_id_,
    });

    api::repositories::family_member::create_family_member(household_id_, new_family_member.into_inner(), &connection)
        .map(|family_member| family_member_created(family_member))
        .map_err(|error| error_status(error))
}

fn family_member_created(family_member: FamilyMember) -> status::Created<Json<FamilyMember>> {
    status::Created(
        format!(
            "{host}:{port}/households/{household_id_}/family_members/{family_member_id_}",
            host=host(),
            port=port(),
            household_id_=family_member.household_id,
            family_member_id_=family_member.id,
        ).to_string(),
        Some(Json(family_member))
    )
}