ALTER TABLE chores DROP CONSTRAINT fk_chores_created_by_family_member;
ALTER TABLE chores DROP CONSTRAINT fk_chores_household;
DROP TABLE chores;
