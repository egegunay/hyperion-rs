# Hyperion

A lightweight key-value store HTTP server written in Rust using the [Warp](https://github.com/seanmonstar/warp) web framework.

## Features

- Simple HTTP API for key-value operations
- In-memory storage using `RwLock<HashMap>` for thread-safe access
- Optional PostgreSQL persistence via SQLx
- Async/await with Tokio runtime

## Prerequisites

- Rust 2024 edition
- (Optional) PostgreSQL database for persistence

## Installation

```bash
git clone https://github.com/egegunay/hyperion-rs.git
cd hyperion-rs
cargo build --release
```

## Usage

### Running the Server

```bash
cargo run
```

The server starts on `http://127.0.0.1:3030`.

### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/insert/{key}` | Insert or increment a key (starts at 1) |
| GET | `/read/{key}` | Read the value of a key |
| GET | `/update/{key}/{value}` | Update a key to a specific value |
| GET | `/delete/{key}` | Delete a key |

### Examples

```bash
# Insert a new key (returns "1")
curl http://127.0.0.1:3030/insert/mykey

# Insert again to increment (returns "2")
curl http://127.0.0.1:3030/insert/mykey

# Read the value
curl http://127.0.0.1:3030/read/mykey

# Update to a specific value
curl http://127.0.0.1:3030/update/mykey/100

# Delete the key
curl http://127.0.0.1:3030/delete/mykey
```

## Database Setup (Optional)

For PostgreSQL persistence, set the `DATABASE_URL` environment variable:

```bash
export DATABASE_URL=postgres://user:password@localhost/hyperion
```

Create the required table:

```sql
CREATE TABLE kv_store (
    key TEXT PRIMARY KEY,
    value BIGINT NOT NULL
);
```

## Testing

```bash
cargo test
```

## License

This project is open source.
