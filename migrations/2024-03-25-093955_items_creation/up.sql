create table items (
    id serial primary key,
    "name" varchar(64) unique not null,
    "description" varchar(128) not null,
    picture varchar(256),
    price float8 not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);