CREATE TABLE loads.load (
    id UUID PRIMARY KEY,
    shipper_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE loads.stop (
    id UUID PRIMARY KEY,
    load_id UUID NOT NULL REFERENCES loads.load (id),
    kind TEXT NOT NULL,
    date DATE NOT NULL,
    name TEXT,
    line1 TEXT NOT NULL,
    line2 TEXT,
    city TEXT NOT NULL,
    region TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    country TEXT NOT NULL,
    UNIQUE (load_id, kind)
);

CREATE TABLE loads.idempotency_key (
    actor_id TEXT NOT NULL,
    key TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    outcome TEXT NOT NULL,
    UNIQUE (actor_id, key)
);
