-- Copyright (C) 2024 Dirk Strauss
--
-- This file is part of Nachtwacht.
--
-- Nachtwacht is free software: you can redistribute it and/or modify
-- it under the terms of the GNU General Public License as published by
-- the Free Software Foundation, either version 3 of the License, or
-- (at your option) any later version.
--
-- Nachtwacht is distributed in the hope that it will be useful,
-- but WITHOUT ANY WARRANTY; without even the implied warranty of
-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
-- GNU General Public License for more details.
--
-- You should have received a copy of the GNU General Public License
-- along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
