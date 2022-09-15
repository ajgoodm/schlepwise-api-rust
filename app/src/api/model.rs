#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::{family_members, households};

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "households"]
pub struct Household {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="households"]
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
#[table_name="family_members"]
pub struct NewFamilyMember {
    pub first_name: String,
    pub last_name: Option<String>,
    pub household_id: i32,
}
