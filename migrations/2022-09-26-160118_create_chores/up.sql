CREATE TABLE chores (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR,
  expected_duration_minutes REAL,
  household_id SERIAL NOT NULL,
  created_by_family_member_id SERIAL,
  CONSTRAINT fk_chores_household
    FOREIGN KEY(household_id)
    REFERENCES households(id),
  CONSTRAINT fk_chores_created_by_family_member
    FOREIGN KEY(created_by_family_member_id)
    REFERENCES family_members(id)
);
