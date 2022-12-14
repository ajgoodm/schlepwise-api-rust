#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::api::model::{Household, NewHousehold};

use crate::schema::family_members;
use crate::schema::households;

pub fn create_household(
    new_household: NewHousehold,
    conn: &PgConnection,
) -> QueryResult<Household> {
    diesel::insert_into(households::table)
        .values(&new_household)
        .get_result(conn)
}

pub fn show_households(connection: &PgConnection) -> QueryResult<Vec<Household>> {
    households::table.limit(5).load::<Household>(connection)
}

pub fn get_household(household_id_: i32, connection: &PgConnection) -> QueryResult<Household> {
    households::table
        .find(household_id_)
        .get_result::<Household>(connection)
}

pub fn update_household(
    household_id_: i32,
    household: Household,
    connection: &PgConnection,
) -> QueryResult<Household> {
    diesel::update(households::table.find(household_id_))
        .set(&household)
        .get_result(connection)
}

pub fn delete_household(household_id_: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(family_members::table.filter(family_members::household_id.eq(household_id_)))
        .execute(connection)?;
    diesel::delete(households::table.find(household_id_)).execute(connection)
}
