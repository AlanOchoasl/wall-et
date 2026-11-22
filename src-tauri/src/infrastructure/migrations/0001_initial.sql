CREATE TABLE service (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    total_cost_minor INTEGER NOT NULL,
    currency_code TEXT NOT NULL,
    shared_people_count INTEGER NOT NULL,
    notes TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE investment (
    id TEXT PRIMARY KEY,
    month TEXT NOT NULL,
    capital_minor INTEGER NOT NULL,
    currency_code TEXT NOT NULL,
    tna_basis_points INTEGER NOT NULL,
    term_days INTEGER NOT NULL,
    tax_minor INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE app_setting (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
