create table users(
    username text primary key not null,
    password varchar(16) not null,
    first_name name not null,
    last_name name not null,
    class smallint not null,
    school int not null,
    email varchar(100) not null,
    is_active boolean not null,
    raiting smallint[] not null 
);

create table email_vereficator(
    id uuid primary key not null,
    username text references users(username) not null,
    registration_data date not null
);

create table slots(
    id uuid primary key not null,
    owner text references users(username) not null,
    lesson varchar(2) not null,
    transactions_limit int,
    is_active boolean not null
);

create table transactions(
    id uuid primary key not null,
    sender_slot uuid references slots(id) not null,
    recipient text references users(username) not null,
    passed boolean not null,
    time_of_rigestration time not null,
    date_of_rigestration date not null,
    time_of_ending time,
    date_of_ending date,
    price float not null,
    task text not null    
);

create table tasks(
    id uuid primary key not null,
    owner text  references users(username) not null,
    task text not null,
    subject varchar(2) not null,
    publish_time timestamp not null,
    target_finishing_time timestamp not null, 
    price float not null

);

create table location_data(
    owner text references users(username) primary key not null,
    city text not null,
    country text not null
);

create extension if not exists "uuid-ossp";