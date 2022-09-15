// @generated automatically by Diesel CLI.

diesel::table! {
    family_members (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        household_id -> Int4,
    }
}

diesel::table! {
    households (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(family_members -> households (household_id));

diesel::allow_tables_to_appear_in_same_query!(
    family_members,
    households,
);
