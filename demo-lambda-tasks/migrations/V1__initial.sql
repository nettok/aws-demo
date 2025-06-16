create table users (
    id uuid primary key,
    name varchar(255) not null unique,
    password varchar(255) not null,
    created_at timestamp default current_timestamp
);

create table entries (
    id uuid not null,       -- must be UUID v7 to preserve insertion order
    user_id uuid not null,
    date date not null,
    content text not null,
    created_at timestamp default current_timestamp,
    primary key (user_id, date, id),
    constraint fk_user foreign key (user_id) references users(id) on delete cascade
);
