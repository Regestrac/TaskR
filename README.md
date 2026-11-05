# TaskR

A small task-management REST API built in Rust with [Axum](https://github.com/tokio-rs/axum) and Tokio.

Tasks are stored in memory behind an `Arc<Mutex<...>>`, so data persists for the lifetime of the server process and is reset when the server restarts.

## Features

- List, create, read, update, and delete tasks
- JSON request/response bodies
- Shared, thread-safe in-memory state
- Health check endpoint
- Seeded with three example tasks on startup

## Requirements

- Rust (edition 2024, Rust 1.85 or newer)
- Cargo

## Getting Started

```bash
cargo run
```

The API starts on <http://localhost:3000>.

For a release build:

```bash
cargo build --release
./target/release/taskr
```

## API

Base URL: `http://localhost:3000`

| Method | Path          | Description            | Success |
| ------ | ------------- | ---------------------- | ------- |
| GET    | `/health`     | Health check           | 200     |
| GET    | `/tasks`      | List all tasks         | 200     |
| POST   | `/tasks`      | Create a task          | 201     |
| GET    | `/tasks/{id}` | Get a single task      | 200     |
| PATCH  | `/tasks/{id}` | Update a task          | 200     |
| DELETE | `/tasks/{id}` | Delete a task          | 204     |

Missing tasks return `404 Not Found`.

### Task schema

```json
{
  "id": 1,
  "title": "Learn Rust",
  "done": false
}
```

### Examples

List tasks:

```bash
curl http://localhost:3000/tasks
```

Create a task:

```bash
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Write documentation"}'
```

Get a task:

```bash
curl http://localhost:3000/tasks/1
```

Update a task (all fields optional):

```bash
curl -X PATCH http://localhost:3000/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust properly", "done": true}'
```

Delete a task:

```bash
curl -X DELETE http://localhost:3000/tasks/1
```

## Project Structure

```
src/
├── main.rs            # Application entry point and server startup
├── routes.rs          # Router and endpoint definitions
├── state.rs           # Shared in-memory application state
├── models.rs          # Task and request/response types
└── handlers/
    └── tasks.rs       # CRUD handler implementations
```

## Notes

- State is non-persistent; restarting the server recreates the seed data.
- The initial `next_id` is `4`, since tasks 1-3 are created at startup.
