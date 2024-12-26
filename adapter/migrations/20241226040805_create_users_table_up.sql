create table users (
    id text SERIAL PRIMARY KEY,
    email text NOT NULL,
    name text NOT NULL,
    password_hash text NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);