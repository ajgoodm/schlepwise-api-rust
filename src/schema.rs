// @generated automatically by Diesel CLI.

diesel::table! {
    chores (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        expected_duration_minutes -> Nullable<Float4>,
        household_id -> Int4,
        created_by_family_member_id -> Int4,
    }
}

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

diesel::joinable!(chores -> family_members (created_by_family_member_id));
diesel::joinable!(chores -> households (household_id));
diesel::joinable!(family_members -> households (household_id));

diesel::allow_tables_to_appear_in_same_query!(chores, family_members, households,);
