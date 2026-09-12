# What compose file boots local Postgres?

Type: grilling
Label: wayfinder:grilling
Status: resolved

Blocked by:

## Question

Pin the local Postgres compose file this workspace runs against.

Hive law: always-on Postgres. Compose locally. Tests never start Postgres. Testcontainers is out. Ops provisions the database, both LOGINs, `CREATE SCHEMA … AUTHORIZATION` migrator, GRANTs, and default privileges. Cell migrations never `CREATE ROLE` or `GRANT`. One database. Schema per cell. `REVOKE ALL ON SCHEMA public FROM PUBLIC`. This map has one product cell. GRANT proof plants a second schema and expects SQLSTATE `42501`.

Decide the compose file path, Postgres image, host port, how init creates the loads schema and both LOGINs, and the planted GRANT-proof schema.

Do not pick DSN env names. Do not pick Load fields. Do not add Redis.

## Answer

Repo-root `compose.yaml`. Init is `postgres/init.sql`, mounted to `/docker-entrypoint-initdb.d/01-init.sql`. Named volume. `pg_isready` healthcheck. Postgres only.

Image is `postgres:18`. Official. Major tag. Patch floats.

Service name is `postgres`. Host bind is `127.0.0.1:5432`. Database is `hive`. Image superuser is `postgres` / `postgres`. App env never uses it.

Schema `loads`. LOGIN `loads_migrator` owns it. LOGIN `loads` is DML only. Local passwords match the role names. Init grants `CONNECT` on `hive`, `USAGE` on `loads`, table `SELECT, INSERT, UPDATE, DELETE`, sequence `USAGE, SELECT`, and the matching `ALTER DEFAULT PRIVILEGES FOR ROLE loads_migrator`. No `CREATE` on `loads` for the cell role. `REVOKE ALL ON SCHEMA public FROM PUBLIC`. Belt: `ALTER ROLE loads SET search_path = loads`. Same belt on `loads_migrator`.

Schema `grant_proof`. LOGIN `grant_proof` owns it. Password matches the role. Init creates `grant_proof.probe`. `loads` gets no `USAGE`. No migrator LOGIN. Not a cell. This ticket does not name that pool's env var.

## Comments

### Round 1

Five arrows accepted:

- Q1 A: repo-root `compose.yaml`. Init at `postgres/init.sql`. Named volume. `pg_isready`. Postgres only.
- Q2 A: `postgres:18`. Official. Major tag.
- Q3 A: service `postgres`. Host `127.0.0.1:5432`. Database `hive`. Superuser `postgres` / `postgres`. Not an app DSN.
- Q4 A: schema `loads`. LOGINs `loads_migrator` and `loads`. Passwords match the role names. Ops GRANTs plus `search_path` belt.
- Q5 A: schema `grant_proof`. LOGIN `grant_proof`. Table `grant_proof.probe`. Not a cell. No env name here.

Frontier empty for this ticket. Redis is not compose. Worker databases wait on [Prove nextest profiles and GRANT](09-nextest-and-grant.md).


