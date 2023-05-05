-- Add migration script here

ALTER TABLE "repos" DROP COLUMN "install";

DROP TABLE "installations";
ALTER TABLE "job" DROP COLUMN "repo";
DROP TABLE "repos";

CREATE TABLE IF NOT EXISTS "repo" (
    "id" BIGSERIAL PRIMARY KEY,
    "name" VARCHAR (128) NOT NULL,
    "git_url" VARCHAR,
    "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc')
);