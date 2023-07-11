# chat-rs

Experimental project to build a chat application server in Rust

# Table of Contents

- [Pre-requisite](#pre-requisite)
- [Usage Guide](#usage-guide)
  - [Database Setup](#database-setup)
- [Useful Commands](#useful-commands)
  - [sqlx-cli](#sqlx-cli)
  - [PostgreSQL](#postgresql)
    - [Dump database using pg_dump](#dump-database-in-docker-container-using-pg_dump)
    - [Restore database using pg_restore](#restore-database-to-docker-container-using-pg_restore)
  - [Redis](#redis)

## Pre-requisite

Rust

```bash
❯ rustup --version
rustup 1.26.0 (5af9b9484 2023-04-05)
❯ rustc --version
rustc 1.70.0 (90c541806 2023-05-31)
```

[Docker Desktop - Mac M1 version .dmg](https://desktop.docker.com/mac/main/arm64/Docker.dmg?utm_source=docker&utm_medium=webreferral&utm_campaign=docs-driven-download-mac-arm64)

## Usage Guide

### Database Setup

- Setup local docker - PostgreSQL container

```bash
docker pull postgres:15.3
docker run -d -e POSTGRES_USER=root -e POSTGRES_PASSWORD=root -p 5432:5432 -v /var/lib/postgresql/data/postgres:/var/lib/postgresql/data --name postgres postgres:15.3
```

- Setup initial database in PostgreSQL container (Optional if you don't need to alter database name)

```bash
docker exec -it tenant-first-pg bash
psql -U tenantfirst -d postgres
drop database tenantfirst;
create database tenant_first;
```

## Useful Commands

### sqlx-cli

We will be using sqlx-cli, which is SQLx's associated command-line utility for managing database migrations

All commands require that a database url is provided. This can be done either with the `--database-url` command line option or by setting `DATABASE_URL`, either in the environment or in a `.env` file in the current working directory.

```bash
# Only install postgres related code
cargo add sqlx-cli --no-default-features -F postgres
# Creating reversible migrations in migrations directory
sqlx migrate add -r <name>
# Apply migration (default to be using .env in same directory you running the command)
sqlx migrate run
# Apply migration with explicit database url
sqlx migrate run --database-url postgresql://postgres:postgres@localhost:5432/postgres
# Revert migration
sqlx migrate revert
```

### PostgreSQL

We will be using a powerful cli tools to manage our postgres database - `pgcli`

```bash
brew install pgcli
vim ~/.config/pgcli/config
# A config file is automatically created at ~/.config/pgcli/config at first launch
# See the file itself for a description of all available options
# Add alias dsn config under the section like below
[alias_dsn]
# example_dsn = postgresql://[user[:password]@][netloc][:port][/dbname]
pgcli -D <name>
```

Please have `libpq` installed first before moving to the following commands

```bash
brew install libpq
vim ~/.zshrc
# Add export instruction inside ~/.zshrc
export PATH=$PATH:/opt/homebrew/opt/libpq/bin
```

#### Dump database in docker container using `pg_dump`

```bash
# docker exec -i <docker-container-name> /bin/bash -c "PGPASSWORD=<password> pg_dump --username <username> <database> > <path-to-dumped-file-on-host-machine>
docker exec -i tenant-first-pg /bin/bash -c "PGPASSWORD=tenantfirst pg_dump --username tenantfirst tenant_first" > ~/Downloads/tenant-first-pg-dump.sql
```

#### Restore database to docker container using `pg_restore`

```bash
# docker exec -i <docker-container-name> pg_restore --verbose --clean --no-acl --no-owner -U <username> -d <database> < <path-to-dumped-file-on-host-machine>
docker exec -i tenant-first-pg pg_restore --verbose --clean --no-acl --no-owner -U tenantfirst -d tenant_first < ~/Downloads/tenant-first-pg-dump.sql
```

### Redis

We will be using a powerful cli tools to manage our redis - `iredis`

```bash
brew install iredis
vim ~/.iredisrc
# Add alias dsn config inside ~/.iredisrc
[alias_dsn]
<name>=redis://<username>:<password>@<host>:<port>
iredis -d <name>
```

