#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::api::model::{Chore, FamilyMember, Household, NewChore};

use crate::schema::chores;
use crate::schema::chores::dsl::*;
use crate::schema::family_members;
use crate::schema::households;

pub fn show_chores(connection: &PgConnection, household_id_: i32) -> QueryResult<Vec<Chore>> {
    households::table
        .find(household_id_)
        .get_result::<Household>(connection)?;
    chores
        .filter(household_id.eq(&household_id_))
        .limit(5)
        .load::<Chore>(connection)
}

pub fn create_chore(
    household_id_: i32,
    new_chore: NewChore,
    connection: &PgConnection,
) -> QueryResult<Chore> {
    households::table
        .find(household_id_)
        .get_result::<Household>(connection)?;
    family_members::table
        .find(new_chore.created_by_family_member_id)
        .get_result::<FamilyMember>(connection)?;
    diesel::insert_into(chores::table)
        .values(&new_chore)
        .get_result(connection)
}

pub fn get_chore(
    household_id_: i32,
    chore_id: i32,
    connection: &PgConnection,
) -> QueryResult<Chore> {
    chores::table
        .find(chore_id)
        .filter(household_id.eq(&household_id_))
        .get_result(connection)
}

pub fn delete_chore(
    household_id_: i32,
    chore_id: i32,
    connection: &PgConnection,
) -> QueryResult<usize> {
    households::table
        .find(household_id_)
        .get_result::<Household>(connection)?;
    diesel::delete(chores.find(chore_id)).execute(connection)
}
