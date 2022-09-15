#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::api::model::{Household, NewHousehold};

use crate::schema::households;
use crate::schema::households::dsl::*;

pub fn create_household(new_household: NewHousehold, conn: &PgConnection) -> QueryResult<Household> {
    diesel::insert_into(households::table)
        .values(&new_household)
        .get_result(conn)
}

pub fn show_households(connection: &PgConnection) -> QueryResult<Vec<Household>>  {
    households.limit(5).load::<Household>(&*connection)
}

pub fn get_household(household_id: i32, connection: &PgConnection) -> QueryResult<Household> {
    households::table.find(household_id).get_result::<Household>(connection)
}

pub fn update_household(household_id: i32, household: Household, connection: &PgConnection) -> QueryResult<Household> {
    diesel::update(households::table.find(household_id))
        .set(&household)
        .get_result(connection)
}

pub fn delete_household(household_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(households::table.find(household_id))
        .execute(connection)
}
