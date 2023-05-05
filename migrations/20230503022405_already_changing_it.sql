-- Add migration script here

ALTER TABLE "objects" 
DROP COLUMN "steps",
DROP COLUMN "last_ran_job", 
ADD COLUMN "refers_to" BIGINT;

CREATE TABLE IF NOT EXISTS "pipelines" (
    "id" BIGSERIAL PRIMARY KEY,
    "name" VARCHAR(128) NOT NULL,
    "steps" BIGINT[] NOT NULL DEFAULT '{}',
    "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc')
);

CREATE TABLE IF NOT EXISTS "pipeline_step" (
    "id" BIGSERIAL PRIMARY KEY,
    "name" VARCHAR(128),
    "run" VARCHAR,
    "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc')
);