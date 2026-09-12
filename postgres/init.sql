REVOKE ALL ON SCHEMA public FROM PUBLIC;

CREATE ROLE loads_migrator LOGIN PASSWORD 'loads_migrator';
CREATE ROLE loads LOGIN PASSWORD 'loads';
CREATE SCHEMA loads AUTHORIZATION loads_migrator;

GRANT CONNECT ON DATABASE hive TO loads_migrator;
GRANT CONNECT ON DATABASE hive TO loads;
GRANT USAGE ON SCHEMA loads TO loads;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA loads TO loads;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA loads TO loads;

ALTER DEFAULT PRIVILEGES FOR ROLE loads_migrator IN SCHEMA loads
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO loads;
ALTER DEFAULT PRIVILEGES FOR ROLE loads_migrator IN SCHEMA loads
    GRANT USAGE, SELECT ON SEQUENCES TO loads;

ALTER ROLE loads SET search_path = loads;
ALTER ROLE loads_migrator SET search_path = loads;

CREATE ROLE grant_proof LOGIN PASSWORD 'grant_proof';
CREATE SCHEMA grant_proof AUTHORIZATION grant_proof;
CREATE TABLE grant_proof.probe (
    id integer PRIMARY KEY
);
ALTER TABLE grant_proof.probe OWNER TO grant_proof;
