-- Copyright (C) 2023 Dirk Strauss
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

insert into state(name, created_by)
values ('Draft', '2146a93a-afa4-4d05-908e-7573a3e39118')
     , ('New', '2146a93a-afa4-4d05-908e-7573a3e39118')
     , ('Active', '2146a93a-afa4-4d05-908e-7573a3e39118')
     , ('Inactive', '2146a93a-afa4-4d05-908e-7573a3e39118')
     , ('Deleted', '2146a93a-afa4-4d05-908e-7573a3e39118')
;
select *
from state;

insert into account(id, name, email, state, created_by)
values ('2146a93a-afa4-4d05-908e-7573a3e39118', 'Master Account', 'email@some.mail.server', (select id
                                                                                             from state
                                                                                             where name like 'Active'),
        '2146a93a-afa4-4d05-908e-7573a3e39118');

insert into roles(id, name, state, created_by)
values (uuid_generate_v4(), 'SuperAdmin', (select id from state where name like 'Active'),
        '2146a93a-afa4-4d05-908e-7573a3e39118');

insert into roles_to_accounts(account, role, created_by)
values ('2146a93a-afa4-4d05-908e-7573a3e39118',
        (select id from roles where name like 'SuperAdmin'),
        '2146a93a-afa4-4d05-908e-7573a3e39118');
