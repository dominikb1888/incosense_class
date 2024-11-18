#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.7' sqlx-cli\--no-default-features --features rustls, postgres"
  echo >&2 "to install it."
  exit 1
fi


DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"


if [ ! -f "$PGDATA/PG_VERSION" ] ; then
  initdb
fi

# Start PostgreSQL using pg_ctl if not running
if ! pg_isready -q; then
  echo "Starting PostgreSQL with pg_ctl..."
  pg_ctl start
fi

# Ensure the postgres superuser exists
if ! psql -U postgres -c '\du' | grep -qw postgres; then
  echo "Creating postgres superuser..."
  createuser -U postgres -s postgres
  pg_ctl restart
else
  echo "Postgres superuser already exists."
fi

# Check if the specified database exists, create if not
if ! psql -U postgres -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
  echo "Creating database: $DB_NAME"
  createdb -U postgres "$DB_NAME"
else
  echo "Database $DB_NAME already exists."
fi

until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port${DB_PORT}!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}

export DATABASE_URL


sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
