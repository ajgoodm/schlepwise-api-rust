#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::households;

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
