CREATE TABLE waifu_databases (
    id              SERIAL      PRIMARY KEY,

    -- secret is secret key each waifu has to authenticate themselved
    -- so orchestrator can trust them when they ask about database password
    hashed_secret   TEXT        NOT NULL UNIQUE,
    waifu_name      TEXT        NOT NULL UNIQUE, -- only one database per waifu

    -- each waifu will have their own database and user.
    db_name         TEXT        NOT NULL UNIQUE,
    db_user         TEXT        NOT NULL UNIQUE,
    db_password     TEXT        NOT NULL,
    created_at      TIMESTAMP   DEFAULT NOW() NOT NULL
)
