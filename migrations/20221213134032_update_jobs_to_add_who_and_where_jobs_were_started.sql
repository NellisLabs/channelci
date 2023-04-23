-- Add migration script here

ALTER TABLE "job" 
ADD COLUMN "triggered_by" VARCHAR NOT NULL, 
ADD COLUMN "start" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
ADD COLUMN "end" TIMESTAMP WITHOUT TIME ZONE;
