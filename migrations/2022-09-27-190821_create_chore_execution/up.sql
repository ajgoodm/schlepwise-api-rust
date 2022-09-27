CREATE TABLE chore_executions (
  id SERIAL PRIMARY KEY,
  started_at TIMESTAMPTZ,
  finished_at TIMESTAMPTZ,
  chore_id SERIAL,
  executed_by_family_member_id SERIAL,
  CONSTRAINT fk_chore_executed
    FOREIGN KEY(chore_id)
    REFERENCES chores(id),
  CONSTRAINT fk_chore_executed_by_family_member
    FOREIGN KEY(executed_by_family_member_id)
    REFERENCES family_members(id)
);
