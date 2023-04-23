-- Add migration script here

ALTER TABLE "runners" 
DROP COLUMN "local_path", 
ADD COLUMN "token" VARCHAR NOT NULL;