# Rust REST API Example

A well-structured REST API built with Rust and Axum to demonstrate best practices for beginners.

## 🏗️ Project Structure

```
rust-api-example/
├── Cargo.toml          # Dependencies and project metadata
└── src/
    ├── main.rs         # Application entry point
    ├── state.rs        # Shared application state
    ├── models.rs       # Data structures (User, requests, responses)
    ├── handlers.rs     # Request handlers (business logic)
    └── routes.rs       # Route definitions
```

## 🔑 Key Rust Concepts Demonstrated

### 1. **Ownership & Borrowing** (handlers.rs)
```rust
let users = state.users.read().await;  // Borrow (read lock)
let mut users = state.users.write().await;  // Mutable borrow (write lock)
```

### 2. **Async/Await** (main.rs, all handlers)
- `#[tokio::main]`: Sets up async runtime
- `async fn`: Asynchronous functions
- `.await`: Waits for async operations to complete

### 3. **Error Handling** (handlers.rs)
```rust
Result<Json<ApiResponse<User>>, StatusCode>
match users.get(&id) {
    Some(user) => Ok(...),
    None => Err(StatusCode::NOT_FOUND),
}
```

### 4. **Traits & Derives** (models.rs)
```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
// Serialize: Convert to JSON
// Deserialize: Parse from JSON
// Clone: Allow copying
// Debug: Enable debug printing
```

### 5. **Concurrency** (state.rs)
- `Arc`: Thread-safe reference counting
- `RwLock`: Multiple readers OR single writer

### 6. **Pattern Matching** (handlers.rs)
```rust
match users.get(&id) {
    Some(user) => /* handle found */,
    None => /* handle not found */,
}
```

## 📦 Dependencies Explained

| Crate | Purpose |
|-------|---------|
| **axum** | Modern web framework (routing, extractors) |
| **tokio** | Async runtime (handles async/await) |
| **serde** | Serialization/deserialization (JSON ↔ structs) |
| **tower** | Middleware and service utilities |
| **tracing** | Structured logging |
| **uuid** | Unique ID generation |

## 🚀 Running the API

### Prerequisites
Install Rust: https://rustup.rs/

### Build and Run
```bash
cd rust-api-example

# Build the project
cargo build

# Run the server
cargo run
```

The server starts on `http://127.0.0.1:3000`

## 🧪 Testing the API

### Health Check
```bash
curl http://localhost:3000/health
```

### Create a User
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Alice",
    "email": "alice@example.com",
    "age": 30
  }'
```

### Get All Users
```bash
curl http://localhost:3000/users
```

### Get Single User
```bash
# Replace {id} with actual UUID from create response
curl http://localhost:3000/users/{id}
```

### Update User
```bash
curl -X PUT http://localhost:3000/users/{id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Alice Updated",
    "age": 31
  }'
```

### Delete User
```bash
curl -X DELETE http://localhost:3000/users/{id}
```

## 🎓 Learning Path

1. **Start with `main.rs`**: See how the server bootstraps
2. **Read `models.rs`**: Understand data structures
3. **Study `state.rs`**: Learn shared state management
4. **Explore `handlers.rs`**: See how requests are processed
5. **Check `routes.rs`**: Understand routing

## 🔍 Important Rust Patterns

### Extractors (Axum concept)
Axum automatically extracts data from requests:
```rust
State(state): State<AppState>        // Shared app state
Path(id): Path<Uuid>                 // URL path parameter
Json(payload): Json<CreateUserRequest> // JSON request body
```

### Result Type
```rust
Result<T, E>  // Either Ok(T) or Err(E)
// Used for operations that can fail
```

### Option Type
```rust
Option<T>  // Either Some(T) or None
// Used for values that may or may not exist
```

### Lifetimes (not heavily used here, but important)
Rust ensures references are always valid. Lifetimes are usually inferred.

## 🚨 Common Rust "Gotchas" for Beginners

1. **Ownership**: Each value has one owner; when owner goes out of scope, value is dropped
2. **Borrowing Rules**: Either one mutable reference OR many immutable references
3. **Explicit Error Handling**: No exceptions; use `Result` and `Option`
4. **No null**: Use `Option<T>` instead
5. **Immutable by default**: Variables are immutable unless declared with `mut`

## 📚 Next Steps

- Add a real database (PostgreSQL with SQLx or Diesel)
- Add authentication (JWT tokens)
- Add validation (validator crate)
- Add tests (Rust has built-in test framework)
- Add API documentation (utoipa crate for OpenAPI)
- Add migrations (sqlx-cli or diesel-cli)

## 📖 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
