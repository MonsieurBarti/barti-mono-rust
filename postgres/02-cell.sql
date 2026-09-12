CREATE SCHEMA loads AUTHORIZATION loads_migrator;

DO $$
BEGIN
    EXECUTE format('GRANT CONNECT ON DATABASE %I TO loads_migrator', current_database());
    EXECUTE format('GRANT CONNECT ON DATABASE %I TO loads', current_database());
    EXECUTE format('GRANT CONNECT ON DATABASE %I TO pgbot_ro', current_database());
END
$$;

GRANT USAGE ON SCHEMA loads TO loads;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA loads TO loads;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA loads TO loads;

ALTER DEFAULT PRIVILEGES FOR ROLE loads_migrator IN SCHEMA loads
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO loads;
ALTER DEFAULT PRIVILEGES FOR ROLE loads_migrator IN SCHEMA loads
    GRANT USAGE, SELECT ON SEQUENCES TO loads;

CREATE SCHEMA grant_proof AUTHORIZATION grant_proof;
CREATE TABLE grant_proof.probe (
    id integer PRIMARY KEY
);
ALTER TABLE grant_proof.probe OWNER TO grant_proof;
