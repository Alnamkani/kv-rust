# ADR-001: Web API Endpoint Design

**Status**: Proposed  
**Date**: 2026-01-21  
**Deciders**: Project Team  
**Related**: Future PRD for HTTP API implementation

## Context

The kv-rust project needs to expose its key-value store functionality through a web API. Currently, the codebase has a basic `/health` endpoint and a preliminary `/read/{key}` endpoint using action-based routing. We need to standardize our API design to ensure:

- Consistency across all endpoints (read, write, delete operations)
- RESTful best practices for resource management
- Clear semantics for different operation types (create vs update)
- Extensibility for future features (metadata, batch operations, etc.)
- Machine-readable error responses for client applications
- Proper HTTP status code usage

The API will be built using actix-web 4.x and needs to work well with JSON serialization (serde/serde_json) and the existing validation framework (validator crate).

## Decision

We will implement a **resource-based REST API** with the following design:

### Endpoint Structure

```
GET    /keys/{key}           - Retrieve a key-value pair
POST   /keys                 - Create a new key-value pair (409 if exists)
PUT    /keys/{key}           - Update/create (upsert) a key-value pair
DELETE /keys/{key}           - Delete a key-value pair
GET    /health               - Health check endpoint
```

### Request/Response Formats

#### Response Model Architecture

We use a three-type model structure for clean separation of concerns:

1. **KeyValue** - Contains just the key-value data
2. **Metadata** - Contains timestamps and other metadata
3. **KeyValueResponse** - Combines both with metadata nested under `metadata` key

This design:
- Keeps core data separate from metadata
- Allows easy extension of metadata without touching KV structure
- Follows common API patterns (metadata nesting)
- Makes it clear what's data vs. what's system information

#### Successful GET Response (200 OK)
```json
{
  "key": "user:123",
  "value": "John Doe",
  "metadata": {
    "created_at": "2026-01-21T10:30:00Z",
    "updated_at": "2026-01-21T10:30:00Z"
  }
}
```

#### POST Request Body
```json
{
  "key": "user:123",
  "value": "John Doe"
}
```

#### POST Response (201 Created)
```json
{
  "key": "user:123",
  "value": "John Doe",
  "metadata": {
    "created_at": "2026-01-21T10:30:00Z",
    "updated_at": "2026-01-21T10:30:00Z"
  }
}
```

#### PUT Request Body
```json
{
  "value": "Jane Doe"
}
```

#### PUT Response (200 OK or 201 Created)
```json
{
  "key": "user:123",
  "value": "Jane Doe",
  "metadata": {
    "created_at": "2026-01-21T10:30:00Z",
    "updated_at": "2026-01-21T14:45:00Z"
  }
}
```

#### DELETE Response (204 No Content)
Empty body

#### Error Response Format
```json
{
  "error": {
    "code": "KEY_NOT_FOUND",
    "message": "The key 'user:999' does not exist in the store"
  }
}
```

### HTTP Status Codes

- **200 OK** - Successful GET or PUT (update)
- **201 Created** - Successful POST or PUT (create)
- **204 No Content** - Successful DELETE
- **400 Bad Request** - Invalid request body or validation failure
- **404 Not Found** - Key does not exist (GET, DELETE)
- **409 Conflict** - Key already exists (POST only)
- **500 Internal Server Error** - Server-side errors

### Error Codes

Standardized error codes for machine parsing:

- `KEY_NOT_FOUND` - Requested key doesn't exist
- `KEY_ALREADY_EXISTS` - POST attempted on existing key
- `INVALID_REQUEST` - Malformed JSON or missing required fields
- `VALIDATION_ERROR` - Key/value failed validation rules
- `INTERNAL_ERROR` - Unexpected server error

### POST vs PUT Semantics

- **POST /keys** - Create only. Returns 409 Conflict if key exists. Communicates clear intent to create new resource.
- **PUT /keys/{key}** - Upsert (update or insert). Idempotent operation. Returns 200 if updated, 201 if created.

This distinction allows clients to:
1. Prevent accidental overwrites using POST
2. Use idempotent retries with PUT
3. Differentiate between "create new" vs "set value" operations

## Consequences

### Positive

1. **RESTful Standards** - Follows HTTP/REST conventions, familiar to developers
2. **Clear Semantics** - POST vs PUT distinction prevents accidental overwrites
3. **Extensible** - Three-type model (KeyValue, Metadata, Response) allows future enhancements without breaking changes
4. **Separation of Concerns** - Core data (key/value) cleanly separated from system metadata
5. **Machine-Readable Errors** - Error codes enable proper client-side error handling
6. **Type Safety** - JSON structure maps well to Rust structs with serde
7. **Idempotency** - PUT operations can be safely retried
8. **HTTP Standards** - Proper status codes provide semantic meaning
9. **Future-Proof** - Can add metadata fields (version, ttl, tags) without changing KV structure
10. **Composability** - KeyValue and Metadata types can be reused independently

### Negative

1. **Complexity** - POST vs PUT requires documentation and client awareness
2. **Storage Requirement** - Metadata (timestamps) requires extending the storage layer beyond simple key-value
3. **Migration Needed** - Current `/read/{key}` endpoint needs to be changed to `/keys/{key}`
4. **Timestamp Overhead** - Storing and managing created_at/updated_at adds complexity
5. **Error Code Maintenance** - Need to maintain consistent error codes across all operations

### Neutral/Unknown

1. **Performance Impact** - Additional metadata fields may have negligible performance impact, needs benchmarking
2. **Client Adoption** - Unclear how many clients will utilize POST vs PUT distinction
3. **Timestamp Precision** - May need to decide on timestamp format (ISO8601, Unix epoch, etc.)

## Alternatives Considered

### Alternative 1: Action-Based Routing

```
GET    /read/{key}
POST   /write
DELETE /delete/{key}
```

**Pros**:
- Explicit operation names in URL
- Clear intent from URL alone
- Currently partially implemented in codebase

**Cons**:
- Not RESTful, violates HTTP method semantics
- Redundant (URL path duplicates HTTP method)
- Less familiar to API consumers
- Harder to apply standard REST middleware

**Why not chosen**: Violates REST principles and makes the API less standard. HTTP methods already convey the action.

### Alternative 2: RPC-Style API

```
POST /kv/get
POST /kv/set
POST /kv/delete
```

**Pros**:
- All operations use same HTTP method
- Simple client implementation
- Explicit operation names

**Cons**:
- Can't use HTTP caching (all POST)
- Doesn't leverage HTTP semantics
- Not idempotent where it could be
- Harder to implement with standard REST tools

**Why not chosen**: Throws away HTTP's semantic richness and caching capabilities.

### Alternative 3: Simple POST and PUT Only (No POST vs PUT Distinction)

Use only PUT /keys/{key} for all writes.

**Pros**:
- Simpler API surface
- One less concept to document
- Always idempotent

**Cons**:
- No way to prevent accidental overwrites
- Can't distinguish "create new" from "update existing" operations
- Less control for API consumers

**Why not chosen**: Loss of semantic distinction is valuable for preventing bugs in client applications.

### Alternative 4: Plain Text Responses

Return just the value as plain text instead of JSON.

**Pros**:
- Lower payload size
- Simpler for basic use cases
- Faster parsing

**Cons**:
- Can't include metadata
- Harder to extend in the future
- No structured error responses
- Content-type negotiation needed for errors vs success

**Why not chosen**: Lack of extensibility would require breaking API changes later. Modern applications expect JSON APIs.

### Alternative 5: Do Nothing (Keep Current Implementation)

Keep `/read/{key}` endpoint as-is, add write operations incrementally.

**Pros**:
- No migration needed
- Faster initial implementation

**Cons**:
- Inconsistent API design
- Harder to document and maintain
- Technical debt from the start

**Why not chosen**: This project demonstrates best practices; we should design the API correctly from the beginning.

## Implementation Notes

### Technology Stack

- **actix-web 4.x** - Web framework with built-in JSON support
- **serde/serde_json** - JSON serialization already in Cargo.toml
- **validator** - Request validation already in Cargo.toml
- **chrono** - Will need to add for timestamp management (ISO8601 format recommended)

### Data Models

Will need to create:
```rust
use crate::types::Key;

// Core data types
pub struct KeyValue {
    pub key: Key,
    pub value: String,
}

// Metadata type
pub struct Metadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Union type for responses
pub struct KeyValueResponse {
    pub key: Key,
    pub value: String,
    pub metadata: Metadata,
}

// Request models
pub struct CreateKeyRequest {
    pub key: Key,  // Uses existing Key type with built-in validation
    pub value: String,
}

pub struct UpdateKeyRequest {
    pub value: String,
}

// Error models
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}
```

**Note**: The existing `Key` type (`src/types/key.rs`) provides:
- Validation (max 255 chars, alphanumeric + hyphens/underscores)
- Serde serialization/deserialization with automatic validation
- Type safety ensuring only valid keys in the system

### Storage Layer Changes

Current in-memory storage will need to be extended:
- From `HashMap<String, String>` to `HashMap<String, StoredValue>`
- Where `StoredValue` contains: value, created_at, updated_at

### Validation Rules

Leverage existing `validator` crate for:
- Key length limits (e.g., max 256 characters)
- Key character restrictions (alphanumeric, colons, hyphens, underscores)
- Value size limits (e.g., max 1MB)
- Required field validation

### Testing Strategy

1. **Unit tests** - Test each endpoint handler in isolation
2. **Integration tests** - Test full HTTP request/response cycle
3. **Edge cases** - Empty values, special characters, size limits
4. **Idempotency tests** - Verify PUT can be called multiple times safely
5. **Error scenarios** - Verify correct status codes and error responses

### Performance Considerations

- Timestamps add ~16 bytes per entry (2 × u64 for Unix timestamps)
- JSON serialization overhead is acceptable for initial version
- Can add compression later if needed (Accept-Encoding: gzip)

### Security

- Input validation prevents injection attacks
- Key/value size limits prevent memory exhaustion
- No authentication in initial version (can add later)
- CORS handling to be determined based on client requirements

### Migration Path

Current codebase has `/read/{key}` endpoint:
1. Implement new `/keys/{key}` GET endpoint alongside existing
2. Deprecate `/read/{key}` endpoint
3. Remove old endpoint in next major version

Or simply replace immediately since this is early in development.

## References

- [REST API Design Best Practices](https://restfulapi.net/)
- [HTTP Status Code Semantics (RFC 7231)](https://tools.ietf.org/html/rfc7231#section-6)
- [actix-web JSON Documentation](https://actix.rs/docs/extractors/#json)
- Current implementation: `src/app/read_ops.rs:4` (to be migrated)
- Existing validation setup: `Cargo.toml:15` (validator crate)
