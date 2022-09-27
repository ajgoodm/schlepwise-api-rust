// @generated automatically by Diesel CLI.

diesel::table! {
    chore_executions (id) {
        id -> Int4,
        started_at -> Timestamptz,
        finished_at -> Timestamptz,
        chore_id -> Int4,
        executed_by_family_member_id -> Int4,
    }
}

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

diesel::joinable!(chore_executions -> chores (chore_id));
diesel::joinable!(chore_executions -> family_members (executed_by_family_member_id));
diesel::joinable!(chores -> family_members (created_by_family_member_id));
diesel::joinable!(chores -> households (household_id));
diesel::joinable!(family_members -> households (household_id));

diesel::allow_tables_to_appear_in_same_query!(chore_executions, chores, family_members, households,);
