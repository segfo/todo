-- Your SQL goes here
create table posts(
    id integer not null primary key,
    title varchar not null,
    body text,
    done_flag boolean not null default 0
)
