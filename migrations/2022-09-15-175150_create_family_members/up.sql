CREATE TABLE family_members (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR,
  household_id SERIAL NOT NULL,
  CONSTRAINT fk_family_members_household
    FOREIGN KEY(household_id)
    REFERENCES households(id)
);
