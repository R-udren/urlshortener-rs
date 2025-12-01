# urlshortener-rs

A simple URL shortener service written in Rust using the Axum web framework and PostgreSQL as the database.

## Features
- Shorten long URLs
- Redirect short URLs to their original long URLs
- RESTful API


## Prerequisites
- Rust (latest stable version)
- PostgreSQL database
- Docker and Docker Compose (for local development)

## Setup

1. Clone the repository
2. Create a `.env` file based on the provided `.env.example` file and configure your database settings.
3. Run the PostgreSQL database using Docker Compose:
    ```bash
    docker-compose up -d
    ```
4. Run database migrations.
5. Build and run the application:
    ```bash
    cargo run
    ```