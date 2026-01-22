# Storage Service Layer Spec

**Status**: Draft  
**Created**: 2026-01-21  
**Related**: ADR-001 (Web API Endpoint Design)

## Overview

Implement a storage service interface and in-memory implementation to support the REST API endpoints defined in ADR-001. The design allows for future swappable storage backends (file-based, database, etc.).

## Purpose

Enable the REST API to actually store and retrieve key-value pairs with metadata, replacing the current placeholder responses. Design the interface to be backend-agnostic from the start.

## Requirements

### Storage Interface (`src/service/interface.rs`)

Define a trait with the following operations:

1. **get(key)** → `Option<StoredValue>`
   - Retrieve value and metadata by key
   - Returns `None` if key doesn't exist

2. **upsert(key, value)** → `UpsertResult`
   - Create or update a key-value pair
   - Returns whether it was a create or update operation
   - Updates `updated_at` timestamp, preserves `created_at` on updates

3. **delete(key)** → `Option<StoredValue>`
   - Remove a key-value pair
   - Returns the deleted value or `None` if key didn't exist

4. **contains_key(key)** → `bool`
   - Fast check if key exists
   - No need to load the value

5. **list_keys()** → `Vec<Key>` or similar
   - Return all keys in the store (for future admin/debug features)
   - Can return iterator for large datasets

### In-Memory Implementation (`src/service/in_memory.rs`)

Implement the storage interface using:

- **DashMap** for concurrent HashMap with better performance than Arc<Mutex<HashMap>>
- Internal storage: `DashMap<Key, StoredValue>`
- Thread-safe by design (no manual Arc/Mutex needed with DashMap)

### Data Structures

```rust
pub struct StoredValue {
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum UpsertResult {
    Created,
    Updated,
}
```

## Technical Decisions

### Why DashMap?
- Better concurrent performance than Arc<Mutex<HashMap>>
- No lock contention on reads
- Automatic sharding for better scalability
- Well-maintained crate used in production Rust applications

### Why trait-based interface?
- Allows swapping backends without changing API layer
- Easy to add file-based, Redis, or database backends later
- Makes testing easier (can mock the storage layer)

### Operations Design

**Why upsert instead of separate insert/update?**
- Matches PUT endpoint semantics (idempotent)
- Returns result type to indicate create vs update for proper HTTP status codes (201 vs 200)

**POST endpoint behavior:**
- POST will call `contains_key()` first, then `upsert()` if key doesn't exist
- Returns 409 Conflict if key exists (enforces create-only semantics)

**DELETE behavior:**
- Returns the deleted value so endpoint can return it in response if needed
- Or just return success/failure - TBD during implementation

## API Layer Integration

How endpoints will use the storage:

```
GET /keys/{key}
  → storage.get(key)
  → 200 + KeyValueResponse or 404 + ErrorResponse

POST /keys
  → if storage.contains_key(key) → 409 Conflict
  → else storage.upsert(key, value) → 201 Created

PUT /keys/{key}
  → storage.upsert(key, value)
  → 200 OK (updated) or 201 Created (new)

DELETE /keys/{key}
  → storage.delete(key)
  → 204 No Content or 404 Not Found
```

## Dependencies

Add to `Cargo.toml`:
```toml
dashmap = "6.0"
```

## Implementation Steps

1. Define the storage trait in `src/service/interface.rs`
2. Create `StoredValue` and `UpsertResult` types
3. Implement `InMemoryStore` in `src/service/in_memory.rs` using DashMap
4. Add unit tests for all storage operations
5. Create `service/mod.rs` to export the interface and implementation
6. Wire storage into API endpoints in `src/app/`
7. Update endpoints to use storage instead of placeholder data
8. Add error handling (404, 409) based on storage results

## Testing

- Test all CRUD operations work correctly
- Test concurrent access (multiple threads reading/writing)
- Test metadata timestamps (created_at preserved, updated_at changes)
- Test edge cases (empty store, non-existent keys, etc.)

## Success Criteria

- ✅ All REST API endpoints use real storage
- ✅ Data persists within a single server session (in-memory)
- ✅ Thread-safe concurrent access
- ✅ Proper HTTP status codes based on storage results
- ✅ Tests pass for all storage operations
- ✅ Interface allows future backend swaps without changing API code

## Non-Goals

- Persistence across restarts (future feature - requires ADR for storage backend)
- Authentication/authorization
- Data encryption
- Backup/restore functionality

## Future Enhancements

- File-based persistence backend
- Redis backend for distributed systems
- Database backend (PostgreSQL, SQLite)
- Expiration/TTL support
- Key prefix/pattern matching
- Batch operations
- Atomic transactions across multiple keys
