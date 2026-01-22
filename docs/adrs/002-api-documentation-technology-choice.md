# ADR-002: API Documentation Technology Choice

**Status**: Accepted  
**Date**: 2026-01-22  
**Deciders**: Project Team  
**Related**: PRD-001 (Interactive API Documentation)

## Context

The kv-rust project has a fully functional REST API with comprehensive endpoints for CRUD operations on key-value pairs. However, the API lacks formal documentation, which creates several problems:

- **Discovery Gap**: New developers and API consumers must read source code to understand available endpoints
- **Integration Friction**: External users need to manually construct requests without examples or schema validation
- **Testing Overhead**: Developers lack an easy way to test endpoints interactively during development
- **Unprofessional Appearance**: Missing API documentation makes the project look incomplete
- **Maintenance Burden**: No machine-readable API specifications for tooling (code generation, validation, etc.)

We need to choose a technology for generating and serving API documentation that:
- Automatically generates documentation from code (stays in sync)
- Provides interactive testing capabilities
- Supports OpenAPI 3.0+ standard (widely adopted)
- Works well with Actix-web (our web framework)
- Offers compile-time type safety
- Supports multiple documentation UI options

## Decision

We will use **`utoipa`** (https://github.com/juhaku/utoipa) as our OpenAPI documentation generation library, along with its companion UI libraries.

### Technology Stack

**Core Library:**
- `utoipa` v5.x - OpenAPI spec generation using procedural macros

**Documentation UIs:**
- `utoipa-swagger-ui` - Swagger UI (most popular, interactive testing)
- `utoipa-redoc` - Redoc (clean, read-focused)
- `utoipa-rapidoc` - RapiDoc (minimal, fast)
- `utoipa-scalar` - Scalar (modern, polished)

### Implementation Approach

Documentation is generated using Rust procedural macros at compile time:

1. **Model Annotations**: Add `#[derive(ToSchema)]` to all API models
   ```rust
   #[derive(Serialize, Deserialize, ToSchema)]
   pub struct CreateKVRequest {
       #[schema(example = "user-123")]
       pub key: Key,
       #[schema(example = "John Doe")]
       pub value: String,
   }
   ```

2. **Endpoint Annotations**: Add `#[utoipa::path(...)]` to handler functions
   ```rust
   #[utoipa::path(
       post,
       path = "/keys",
       request_body = CreateKVRequest,
       responses(
           (status = 201, description = "Created", body = KeyValueResponse),
           (status = 409, description = "Already exists", body = ErrorResponse)
       ),
       tag = "Keys - Write Operations"
   )]
   #[post("/keys")]
   pub async fn create_kv(...) { }
   ```

3. **OpenAPI Spec Definition**: Create a struct that collects all endpoints and schemas
   ```rust
   #[derive(OpenApi)]
   #[openapi(
       paths(health::health, read_ops::get_value_by_key, ...),
       components(schemas(CreateKVRequest, KeyValueResponse, ...)),
       tags(...),
       info(...)
   )]
   pub struct ApiDoc;
   ```

4. **Mount UI Services**: Serve documentation UIs as Actix services
   ```rust
   App::new()
       .service(SwaggerUi::new("/swagger-ui/{_:.*}")
           .url("/api-docs/openapi.json", ApiDoc::openapi()))
       .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
       // ... other UIs
   ```

### URLs

- `/swagger-ui` - Swagger UI (interactive testing)
- `/redoc` - Redoc UI (clean reading)
- `/rapidoc` - RapiDoc UI (minimal)
- `/scalar` - Scalar UI (modern)
- `/api-docs/openapi.json` - OpenAPI 3.0 spec (JSON)

## Alternatives Considered

### 1. `paperclip`

**Pros:**
- Can auto-generate from code with less boilerplate
- Some automation features

**Cons:**
- Less actively maintained (last major update over a year ago)
- Actix-web support is via older v3 plugin (we use v4)
- Smaller community and ecosystem
- Limited documentation UI options

**Why not chosen:** Maintenance concerns and Actix v4 compatibility issues.

### 2. Manual OpenAPI spec (YAML/JSON) + Swagger UI

**Pros:**
- Full control over the specification
- No compile-time dependencies
- Can use any OpenAPI tooling

**Cons:**
- Manual synchronization between code and docs (high maintenance burden)
- No type safety - errors only caught at runtime
- Documentation easily becomes outdated
- Requires writing repetitive boilerplate

**Why not chosen:** High maintenance burden and lack of type safety makes this impractical for a codebase that values traceability and accuracy.

### 3. External documentation tools (Postman, Stoplight, etc.)

**Pros:**
- Rich UI and collaboration features
- Cloud hosting available
- Team collaboration tools

**Cons:**
- Not self-hosted (external dependency)
- Requires manual synchronization with code
- Additional costs for team features
- Not suitable for open-source projects
- Doesn't integrate with codebase

**Why not chosen:** We want self-hosted documentation that lives with the code and stays in sync automatically.

### 4. `rocket_okapi` (for Rocket framework)

**Pros:**
- Similar functionality to utoipa
- Good Rocket integration

**Cons:**
- Only works with Rocket framework (we use Actix-web)
- Would require switching web frameworks

**Why not chosen:** Framework incompatibility. We're committed to Actix-web (actix-web is more mature and performant).

## Consequences

### Positive

1. **Zero Runtime Cost**: OpenAPI spec generation happens at compile time. No performance impact on the running server.

2. **Type Safety**: Documentation is derived from actual Rust types. If the code changes, the documentation automatically updates. Compiler ensures correctness.

3. **Always In Sync**: Documentation can't drift from implementation because it's generated from the same source.

4. **Multiple UIs**: Users can choose their preferred documentation style (Swagger, Redoc, Scalar, RapiDoc).

5. **Standard Compliance**: Generates OpenAPI 3.0+ spec, compatible with all OpenAPI tooling.

6. **Interactive Testing**: Swagger UI provides "Try it out" functionality for testing endpoints directly from the browser.

7. **Mature Ecosystem**: `utoipa` is actively maintained with 2.4k+ GitHub stars and used in production.

8. **Actix-web First-Class Support**: Built with Actix-web in mind, excellent integration.

### Negative

1. **Macro Annotations Verbosity**: Requires adding `#[utoipa::path(...)]` annotations to every endpoint. This adds code volume, though it's self-documenting.

2. **Compile Time Impact**: More procedural macros may slow down compilation slightly (acceptable trade-off for type safety).

3. **Learning Curve**: Team needs to learn utoipa's macro syntax and conventions.

4. **Breaking Changes Risk**: utoipa is pre-1.0, so API could change (though it's quite stable now at v5).

5. **Binary Size**: Including multiple UI libraries increases binary size by ~5-10MB (acceptable for server deployment).

### Neutral

1. **OpenAPI Limitations**: Some advanced API patterns may be hard to express in OpenAPI (not specific to utoipa).

2. **Maintenance**: Need to keep annotations updated when changing API signatures (but compiler helps enforce this).

## Monitoring & Success Criteria

We'll consider this decision successful if:

- ✅ All REST endpoints are documented with request/response schemas
- ✅ Documentation UIs are accessible and functional
- ✅ OpenAPI spec validates correctly
- ✅ Documentation stays in sync with code (verified by compile-time checks)
- ✅ Developers can test endpoints interactively via Swagger UI
- ✅ Zero security vulnerabilities in documentation dependencies
- 📊 (Future) Documentation page views indicate usage

## Future Considerations

1. **Custom UI**: Could create a custom documentation UI tailored to our needs if the provided UIs don't suffice.

2. **API Versioning**: When we version the API (e.g., `/v1/keys`), we'll need to create separate OpenAPI specs per version.

3. **Static Spec File**: Could generate and commit `openapi.json` to the repository for better visibility in PRs (currently generated at runtime).

4. **Authentication Docs**: When we add authentication, we'll need to document it in the OpenAPI spec (utoipa supports OAuth2, API keys, etc.).

5. **Code Generation**: Could use the OpenAPI spec to generate client SDKs in other languages (TypeScript, Python, etc.).

6. **Spec Validation**: Could add CI checks to validate the generated OpenAPI spec against the official schema.

## Implementation

Implementation was completed in 5 stacked PRs:

1. **feat/api-docs-models**: Added utoipa dependencies and annotated all models with `ToSchema`
2. **feat/api-docs-endpoints**: Added `#[utoipa::path(...)]` to all endpoints
3. **feat/api-docs-swagger**: Created OpenAPI spec and mounted Swagger UI
4. **feat/api-docs-multi-ui**: Added Redoc, RapiDoc, and Scalar UIs
5. **feat/api-docs-polish**: Enhanced descriptions, added examples, updated README, created this ADR

Total implementation time: ~8 hours

## References

- utoipa repository: https://github.com/juhaku/utoipa
- utoipa documentation: https://docs.rs/utoipa/
- OpenAPI Specification: https://spec.openapis.org/oas/latest.html
- Actix-web: https://actix.rs/
- PRD-001: Interactive API Documentation (docs/prds/001-api-documentation.md)

## Revision History

- 2026-01-22: Initial decision document
