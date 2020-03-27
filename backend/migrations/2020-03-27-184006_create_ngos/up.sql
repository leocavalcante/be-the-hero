create table ngos
(
    id        integer primary key autoincrement,
    oid       varchar not null unique,
    name      varchar not null,
    email     varchar not null,
    whats_app varchar not null,
    city      varchar not null,
    state     char(2) not null
);