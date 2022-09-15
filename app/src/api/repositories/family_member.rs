#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::api::model::{FamilyMember, Household};

use crate::schema::households;
use crate::schema::family_members::dsl::*;

pub fn show_family_members(connection: &PgConnection, household_id_: i32) -> QueryResult<Vec<FamilyMember>>  {
    households::table.find(household_id_).get_result::<Household>(connection)?;
    family_members
        .filter(household_id.eq(&household_id_))
        .limit(5)
        .load::<FamilyMember>(&*connection)
}