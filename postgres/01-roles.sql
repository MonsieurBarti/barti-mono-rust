REVOKE ALL ON SCHEMA public FROM PUBLIC;

CREATE ROLE loads_migrator LOGIN PASSWORD 'loads_migrator';
CREATE ROLE loads LOGIN PASSWORD 'loads';
CREATE ROLE grant_proof LOGIN PASSWORD 'grant_proof';
CREATE ROLE pgbot_ro LOGIN PASSWORD 'pgbot_ro';
GRANT pg_monitor TO pgbot_ro;

ALTER ROLE loads SET search_path = loads;
ALTER ROLE loads_migrator SET search_path = loads;
