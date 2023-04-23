-- Add migration script here

CREATE TABLE IF NOT EXISTS "credentials" (
    "id" BIGSERIAL PRIMARY KEY,
    "passphrase" VARCHAR,
    "private_key" VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    "description" VARCHAR,
    "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc')
);

ALTER TABLE "runners"
ADD COLUMN "remote_host" INET,
ADD COLUMN "remote_user" VARCHAR;