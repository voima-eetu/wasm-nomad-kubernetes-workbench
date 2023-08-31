create table users
(
    id bigserial primary key,
    name varchar,
    username varchar,
    ip_address varchar,
    user_agent varchar,
    country varchar,
    city varchar,
    street_name varchar,
    zip_code varchar,
    building_number varchar
);
