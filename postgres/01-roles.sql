REVOKE ALL ON SCHEMA public FROM PUBLIC;

CREATE ROLE loads_migrator LOGIN PASSWORD 'loads_migrator';
CREATE ROLE loads LOGIN PASSWORD 'loads';
CREATE ROLE grant_proof LOGIN PASSWORD 'grant_proof';

ALTER ROLE loads SET search_path = loads;
ALTER ROLE loads_migrator SET search_path = loads;
