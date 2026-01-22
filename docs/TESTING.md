# Testing Guide for KV-Rust

## Test Organization

This project uses **trait-based testing** to ensure all storage implementations behave consistently.

### Test Structure

```
src/
  service/
    mod.rs            - Module declarations
    interface.rs      - Storage trait definition
    in_memory.rs      - InMemoryStorage implementation + concrete tests
    tests.rs          - Generic test suite (tests any Storage implementation)
```

## How It Works

### 1. Generic Test Suite (`src/service/tests.rs`)

Contains **reusable test functions** that work with any `Storage` implementation:

```rust
pub fn test_get_nonexistent_key<S: Storage>(storage: &S) { ... }
pub fn test_upsert_new_key<S: Storage>(storage: &S) { ... }
pub fn test_upsert_existing_key_preserves_created_at<S: Storage>(storage: &S) { ... }
// ... etc
```

These are **generic over any type that implements `Storage`**.

### 2. Concrete Tests (`src/service/in_memory.rs`)

Each storage implementation calls the generic test functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::tests::*;

    #[test]
    fn test_in_memory_get_nonexistent() {
        let storage = InMemoryStorage::new();
        test_get_nonexistent_key(&storage);
    }
    
    // ... more tests
}
```

## Test Coverage

### Storage Operations Tested

✅ **Get Operations**
- `test_get_nonexistent_key` - Returns None for missing keys
- `test_get_existing_key` - Returns value for existing keys

✅ **Upsert Operations**
- `test_upsert_new_key` - Creates new key-value pairs
- `test_upsert_existing_key_preserves_created_at` - **Critical: Ensures `created_at` is preserved on updates**

✅ **Delete Operations**
- `test_delete_existing_key` - Deletes and returns value
- `test_delete_nonexistent_key` - Returns None for missing keys

✅ **Metadata Operations**
- `test_contains_key_exists` - Returns true for existing keys
- `test_contains_key_not_exists` - Returns false for missing keys

✅ **List Operations**
- `test_list_keys_empty` - Empty storage returns empty list
- `test_list_keys_multiple` - Returns all keys

✅ **Concurrency**
- `test_concurrent_upserts` - Thread-safe concurrent access

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Storage Tests Only
```bash
cargo test service::in_memory
```

### Run Specific Test
```bash
cargo test test_in_memory_upsert_preserves_created_at
```

### Run with Output
```bash
cargo test -- --nocapture
```

## Adding New Storage Implementations

When you add a new storage backend (e.g., FileStorage, RedisStorage), follow this pattern:

### Step 1: Implement the Storage Trait

```rust
// src/service/file_storage.rs
pub struct FileStorage {
    // ... implementation
}

impl Storage for FileStorage {
    // ... implement all methods
}
```

### Step 2: Add Concrete Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::tests::*;

    fn create_storage() -> FileStorage {
        FileStorage::new()
    }

    #[test]
    fn test_file_get_nonexistent() {
        let storage = create_storage();
        test_get_nonexistent_key(&storage);
    }
    
    #[test]
    fn test_file_upsert_new() {
        let storage = create_storage();
        test_upsert_new_key(&storage);
    }
    
    // ... call all the generic test functions
}
```

### Step 3: Add Implementation-Specific Tests (Optional)

If your storage has unique behavior (e.g., file persistence), add extra tests:

```rust
#[test]
fn test_file_persistence_across_instances() {
    let path = "/tmp/test.db";
    {
        let storage = FileStorage::new(path);
        storage.upsert(/* ... */);
    }
    // Storage dropped, file should persist
    {
        let storage2 = FileStorage::new(path);
        let result = storage2.get(/* ... */);
        assert!(result.is_some());
    }
}
```

## Why This Approach?

### Benefits

1. **Consistency**: All storage implementations must pass the same tests
2. **DRY**: Write test logic once, reuse for all implementations
3. **Confidence**: Adding a new backend is safe - tests catch regressions
4. **Documentation**: Tests serve as usage examples for the Storage trait

### Example

If you later add a RedisStorage:

```rust
// src/service/redis_storage.rs
pub struct RedisStorage { /* ... */ }
impl Storage for RedisStorage { /* ... */ }

#[cfg(test)]
mod tests {
    // Just call the same generic tests!
    #[test]
    fn test_redis_get_nonexistent() {
        let storage = RedisStorage::new("redis://localhost");
        test_get_nonexistent_key(&storage);  // Same test, different implementation
    }
}
```

The tests **guarantee** Redis behaves identically to InMemory for all core operations.

## Critical Tests

### `test_upsert_existing_key_preserves_created_at`

This is the **most important test** because it catches the bug we fixed earlier:

```rust
// Create a key
let first = storage.upsert(request1);
let original_created_at = first.metadata.created_at;

// Wait a bit
std::thread::sleep(Duration::from_millis(10));

// Update the same key
let second = storage.upsert(request2);

// CRITICAL ASSERTION: created_at must not change
assert_eq!(second.metadata.created_at, original_created_at);

// But updated_at MUST change
assert!(second.metadata.updated_at > second.metadata.created_at);
```

Without this test, you might accidentally overwrite `created_at` timestamps.

## Current Test Results

```
running 11 tests
test service::in_memory::tests::test_in_memory_concurrent_access ... ok
test service::in_memory::tests::test_in_memory_contains_key_exists ... ok
test service::in_memory::tests::test_in_memory_contains_key_not_exists ... ok
test service::in_memory::tests::test_in_memory_delete_existing ... ok
test service::in_memory::tests::test_in_memory_delete_nonexistent ... ok
test service::in_memory::tests::test_in_memory_get_existing ... ok
test service::in_memory::tests::test_in_memory_get_nonexistent ... ok
test service::in_memory::tests::test_in_memory_list_keys_empty ... ok
test service::in_memory::tests::test_in_memory_list_keys_multiple ... ok
test service::in_memory::tests::test_in_memory_upsert_new ... ok
test service::in_memory::tests::test_in_memory_upsert_preserves_created_at ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured
```

All storage tests passing! ✅
