create table if not exists users (
    id uuid primary key,
    account_name varchar(64) not null unique,
    name varchar(64) not null,
    email varchar(64) not null unique,
    password_hash varchar(255),
    is_active boolean default true,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);
