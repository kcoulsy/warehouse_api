# Warehouse API

A REST API for managing warehouses built with Rust, Axum, and SeaORM.

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database

## Setup

1. Clone the repository
2. Copy `.env.example` to `.env` and update the database connection string:
   ```
   cp .env.example .env
   ```
3. Update the `DATABASE_URL` in `.env` with your PostgreSQL credentials:
   ```
   DATABASE_URL=postgresql://username:password@localhost:5432/database_name
   ```

## Running Migrations

To run database migrations:

```bash
cd migration
cargo run -- up
```

## Running the Project

1. Make sure your PostgreSQL database is running
2. Run the migrations (see above)
3. Start the server:

```bash
cargo run
```
