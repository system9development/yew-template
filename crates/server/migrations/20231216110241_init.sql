-- Create users table.
create table if not exists users
(
    id           serial primary key,
    username     text not null unique,
    password     text not null
);