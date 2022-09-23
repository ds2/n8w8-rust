CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table if not exists state
(
    id          serial primary key not null unique,
    name        text               not null unique,
    created     timestamptz        not null default now(),
    created_by  uuid,
    modified    timestamptz,
    modified_by uuid
);

drop table if exists account;
create table if not exists account
(
    id          uuid        not null primary key unique,
    name        text        not null,
    email       text        not null unique,
    state       bigint      not null references state (id),
    created     timestamptz not null default now(),
    created_by  uuid,
    modified    timestamptz,
    modified_by uuid
);

drop table if exists roles;
create table if not exists roles
(
    id          uuid        not null primary key unique default uuid_generate_v4(),
    name        text        not null,
    state       bigint      not null references state (id),
    created     timestamptz not null                    default now(),
    created_by  uuid        not null references account (id),
    modified    timestamptz,
    modified_by uuid references account (id)
);

drop table if exists roles_to_accounts;
create table if not exists roles_to_accounts
(
    id         uuid        not null primary key unique default uuid_generate_v4(),
    account    uuid        not null references account (id),
    role       uuid        not null references roles (id),
    created    timestamptz not null                    default now(),
    created_by uuid        not null references account (id),
    unique (account, role)
);