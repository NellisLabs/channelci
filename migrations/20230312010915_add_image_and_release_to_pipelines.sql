-- Add migration script here

ALTER TABLE "pipelines" 
ADD COLUMN "image" VARCHAR, 
ADD COLUMN "release" VARCHAR,
ADD COLUMN "preferred_runner" VARCHAR REFERENCES "runners"("name");