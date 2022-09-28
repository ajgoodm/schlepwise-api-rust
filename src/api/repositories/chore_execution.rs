#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::api::model::{ChoreExecution, FamilyMember, Household};

use crate::schema::chore_executions;
use crate::schema::chore_executions::dsl::*;
use crate::schema::family_members;
use crate::schema::family_members::dsl::*;
use crate::schema::households;

pub fn show_chore_executions(
    connection: &PgConnection,
    household_id_: i32,
    executed_by_family_member_id_: Option<i32>,
) -> QueryResult<Vec<ChoreExecution>> {
    if let Some(family_member_id_) = executed_by_family_member_id_ {
        family_members::table
            .find(family_member_id_)
            .filter(household_id.eq(&household_id_))
            .get_result::<FamilyMember>(connection)?;
        chore_executions::table
            .filter(executed_by_family_member_id.eq(&family_member_id_))
            .limit(5)
            .load::<ChoreExecution>(connection)
    } else {
        households::table
            .find(household_id_)
            .get_result::<Household>(connection)?;
        chore_executions
            // .inner_join(
            //     family_members::table
            //         .on(family_members::id.eq(chore_executions::executed_by_family_member_id)),
            // )
            // .filter(family_members::household_id.eq(household_id_))
            // .order_by(executed_by_family_member_id.asc())
            .limit(5)
            .load::<ChoreExecution>(connection)
    }
}
