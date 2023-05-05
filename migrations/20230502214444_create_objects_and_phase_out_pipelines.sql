-- Add migration script here

DROP TABLE "pipeline_step";
DROP TABLE "pipelines";

CREATE TABLE IF NOT EXISTS "objects" (
    "id" BIGSERIAL PRIMARY KEY,
    "type" INT NOT NULL,
    "name" VARCHAR(128),
    -- JSONB?
    "steps" JSON[],
    "last_ran_job" BIGINT REFERENCES "job"("id"),
    "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc')
)