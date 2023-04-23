-- Add migration script here

DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'pipelinebackend') THEN
        CREATE TYPE PipelineBackend AS ENUM ('docker','lxc');
    END IF;
END$$;

CREATE TABLE IF NOT EXISTS "pipeline_step" (
    "id" BIGSERIAL PRIMARY KEY,
    "belongs_to" BIGINT NOT NULL REFERENCES "pipelines"("id"),
    "name" VARCHAR NOT NULL,
    "run" VARCHAR NOT NULL
);

ALTER TABLE "pipelines" ADD COLUMN "backend" PipelineBackend, DROP COLUMN "steps";