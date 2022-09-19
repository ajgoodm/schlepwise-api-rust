#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::api::model::{FamilyMember, Household, NewFamilyMember};

use crate::schema::family_members;
use crate::schema::family_members::dsl::*;
use crate::schema::households;

pub fn show_family_members(
    connection: &PgConnection,
    household_id_: i32,
) -> QueryResult<Vec<FamilyMember>> {
    households::table
        .find(household_id_)
        .get_result::<Household>(connection)?;
    family_members
        .filter(household_id.eq(&household_id_))
        .limit(5)
        .load::<FamilyMember>(connection)
}

pub fn create_family_member(
    household_id_: i32,
    new_family_member: NewFamilyMember,
    connection: &PgConnection,
) -> QueryResult<FamilyMember> {
    households::table
        .find(household_id_)
        .get_result::<Household>(connection)?;
    diesel::insert_into(family_members::table)
        .values(&new_family_member)
        .get_result(connection)
}

pub fn get_family_member(
    household_id_: i32,
    family_member_id: i32,
    connection: &PgConnection,
) -> QueryResult<FamilyMember> {
    family_members::table
        .find(family_member_id)
        .filter(household_id.eq(&household_id_))
        .get_result(connection)
}

pub fn delete_family_member(
    household_id_: i32,
    family_member_id: i32,
    connection: &PgConnection,
) -> QueryResult<usize> {
    households::table
        .find(household_id_)
        .get_result::<Household>(connection)?;
    diesel::delete(family_members.find(family_member_id)).execute(connection)
}
