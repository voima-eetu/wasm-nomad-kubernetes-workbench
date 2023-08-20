# Prest Setup

- Create database and setup access

    ```sql
    create database prest;
    create user spin with encrypted password 'spin-secret';
    grant all privileges on database prest to spin;
    ```

- Create the table

    ```sql
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
    ```

- Deploy Prest (`docker-compose.yml`):

    ```yaml
    version: '3.4'
    services:
    prest:
        container_name: prest
        image: prest/prest:v1
        environment:
                - PREST_DEBUG=false
                - PREST_PG_HOST=192.168.85.153
                - PREST_PG_USER=spin
                - PREST_PG_PASS=spin-secret
                - PREST_PG_DATABASE=prest
                - PREST_PG_PORT=5432
                - PREST_JWT_DEFAULT=false
        ports:
        - 3000:3000
    ```

- Start the container: `docker-compose up -d`
- Change the `allowed_http_hosts` parameter in `spin.toml`
- Build the function by executing: `./build.sh`