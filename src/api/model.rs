#![allow(proc_macro_derive_resolution_fallback)]

use chrono::prelude::*;

use crate::schema::{chore_executions, chores, family_members, households};

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "households"]
pub struct Household {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "households"]
pub struct NewHousehold {
    pub name: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "family_members"]
pub struct FamilyMember {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub household_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "family_members"]
pub struct NewFamilyMember {
    pub first_name: String,
    pub last_name: Option<String>,
    pub household_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewFamilyMemberDetails {
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "chores"]
pub struct Chore {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub expected_duration_minutes: Option<f32>,
    pub household_id: i32,
    pub created_by_family_member_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "chores"]
pub struct NewChore {
    pub name: String,
    pub description: Option<String>,
    pub expected_duration_minutes: Option<f32>,
    pub household_id: i32,
    pub created_by_family_member_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewChoreDetails {
    pub name: String,
    pub description: Option<String>,
    pub expected_duration_minutes: Option<f32>,
    pub created_by_family_member_id: i32,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "chore_executions"]
pub struct ChoreExecution {
    pub id: i32,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub chore_id: i32,
    pub executed_by_family_member_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "chore_executions"]
pub struct NewChoreExecution {
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub chore_id: i32,
    pub executed_by_family_member_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewChoreExecutionDetails {
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub chore_id: i32,
    pub executed_by_family_member_id: i32,
}
