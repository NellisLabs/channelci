-- Add migration script here

ALTER TABLE "repo" RENAME TO "project";

ALTER TABLE "pipelines" 
ADD COLUMN "flags" BIGINT DEFAULT 0,
-- https://dba.stackexchange.com/a/207281
-- SELECT ARRAY[1] <@ ARRAY[1,2,3];
ADD COLUMN "projects" BIGINT[] DEFAULT '{}';