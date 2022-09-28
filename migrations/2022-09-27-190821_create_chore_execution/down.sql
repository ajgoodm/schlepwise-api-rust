ALTER TABLE chore_executions DROP CONSTRAINT fk_chore_executed_by_family_member;
ALTER TABLE chore_executions DROP CONSTRAINT fk_chore_executed;
DROP TABLE chore_executions;
