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
