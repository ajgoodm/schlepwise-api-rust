#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::api::model::ChoreExecution;

use crate::schema::chore_executions;
use crate::schema::family_members;

pub fn show_chore_executions_for_household(
    connection: &PgConnection,
    household_id_: i32,
) -> QueryResult<Vec<ChoreExecution>> {
    chore_executions::table
        .filter(
            chore_executions::executed_by_family_member_id.eq_any(
                family_members::table
                    .filter(family_members::household_id.eq(household_id_))
                    .select(family_members::id),
            ),
        )
        .order_by(chore_executions::executed_by_family_member_id.asc())
        .limit(10)
        .load(connection)
}

pub fn show_chore_executions_for_family_member(
    connection: &PgConnection,
    family_member_id_: i32,
) -> QueryResult<Vec<ChoreExecution>> {
    chore_executions::table
        .filter(chore_executions::executed_by_family_member_id.eq(family_member_id_))
        .order_by(chore_executions::chore_id.asc())
        .limit(10)
        .load(connection)
}

pub fn show_chore_executions_for_chore(
    connection: &PgConnection,
    chore_id_: i32,
) -> QueryResult<Vec<ChoreExecution>> {
    chore_executions::table
        .filter(chore_executions::chore_id.eq(chore_id_))
        .order_by(chore_executions::executed_by_family_member_id.asc())
        .limit(10)
        .load(connection)
}
