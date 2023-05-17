-- Add migration script here

CREATE TABLE IF NOT EXISTS "triggers" (
    "id" BIGSERIAL PRIMARY KEY,
    "trigger_type" INT NOT NULL,
    -- If trigger type is 0 (Github) then this MUST be set.
    "github_repo_id" BIGINT,
    "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc')
);

CREATE TABLE IF NOT EXISTS "triggers_used_by" (
    "id" BIGSERIAL PRIMARY KEY,
    "trigger" BIGINT REFERENCES "triggers"("id") NOT NULL,
    "owned_by" BIGINT REFERENCES "pipelines"("id") NOT NULL
);

ALTER TABLE "project" DROP COLUMN "git_url";