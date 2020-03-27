create table projects
(
    id          integer primary key autoincrement,
    ngo_id      integer not null references ngos (id),
    name        varchar not null,
    description varchar not null,
    donation    integer not null
);