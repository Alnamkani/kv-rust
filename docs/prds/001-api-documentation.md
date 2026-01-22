# PRD-001: Interactive API Documentation

**Status**: Draft  
**Owner**: Engineering Team  
**Created**: 2026-01-22  

## Overview

Add interactive API documentation to the KV-Rust REST API using OpenAPI 3.0 and modern documentation UIs. This provides both internal developers and external API consumers with comprehensive, self-service documentation that enables API discovery, reduces support burden, and provides an interactive testing interface.

## Problem Statement

Currently, the KV-Rust API has no formal documentation beyond code comments. This creates several problems:

1. **Discovery Gap**: New developers (internal or external) must read source code to understand available endpoints
2. **Integration Friction**: API consumers need to manually construct requests without examples or schema validation
3. **Testing Overhead**: Developers lack an easy way to test endpoints interactively during development
4. **Unprofessional Appearance**: Missing API docs makes the project look incomplete or immature
5. **Maintenance Burden**: Without machine-readable API specs, there's no way to auto-generate client SDKs or validate requests

**Who experiences this?**
- Internal team members onboarding to the codebase
- External developers integrating with the API
- QA/testing teams needing to validate API behavior
- Future contributors exploring the project

## Goals & Success Metrics

### Goals

1. **Enable Self-Service API Discovery**: Any developer can understand and use the API without asking questions
2. **Provide Interactive Testing**: Users can test endpoints directly from the browser without external tools
3. **Generate Machine-Readable Spec**: OpenAPI spec available for tooling (code generation, validation, etc.)
4. **Maintain Type Safety**: Documentation stays in sync with code through compile-time generation

### Success Metrics

- ✅ All REST endpoints documented with examples
- ✅ Interactive UI accessible at `/swagger-ui` (and alternative UIs)
- ✅ OpenAPI 3.0 spec available at `/api-docs/openapi.json`
- ✅ Documentation includes request/response schemas with types
- ✅ Error responses documented with status codes
- ✅ Zero manual synchronization needed between code and docs
- 📊 (Future) Track page views/usage of documentation endpoints
- 📊 (Future) Measure reduction in "how do I use X endpoint?" questions

### Non-Goals

- **Client SDK Generation**: Not building auto-generated client libraries (though spec enables this later)
- **Authentication Documentation**: No auth yet, so no auth docs (future)
- **Performance Benchmarks**: Not documenting performance characteristics
- **Deployment Guides**: Not covering how to deploy the API
- **Tutorials/Guides**: This is reference docs, not learning materials

## User Stories

**As an internal developer**, I want to quickly see what endpoints exist and their schemas, so that I can integrate features without reading source code.

**As an external API consumer**, I want interactive documentation with "Try it" buttons, so that I can test the API before writing integration code.

**As a QA engineer**, I want to see all possible error responses, so that I can write comprehensive test cases.

**As a CLI tool developer**, I want a machine-readable OpenAPI spec, so that I can auto-generate request validation.

**As a product manager**, I want the API to look professional and complete, so that external users trust it for production use.

## Requirements

### Functional Requirements

#### Must Have (P0) - Critical

1. **OpenAPI 3.0 Spec Generation**
   - Generate spec from code using `utoipa` crate
   - Include all REST endpoints: GET, POST, PUT, DELETE
   - Document request/response schemas
   - Include error responses (404, 409, 400, 500)
   - Spec available as JSON at `/api-docs/openapi.json`

2. **Swagger UI**
   - Serve interactive Swagger UI at `/swagger-ui`
   - Allow "Try it" functionality for all endpoints
   - Display request/response examples
   - Show HTTP status codes and descriptions

3. **Core Endpoint Documentation**
   - `GET /health` - Health check
   - `GET /keys` - List all keys
   - `GET /keys/{key}` - Get value by key
   - `POST /keys` - Create new key-value pair
   - `PUT /keys/{key}` - Update or create key-value pair
   - `DELETE /keys/{key}` - Delete key-value pair

4. **Schema Documentation**
   - `CreateKVRequest` - Request body for POST
   - `UpdateKVRequest` - Request body for PUT
   - `KeyValueResponse` - Response with key, value, and metadata
   - `ValueResponse` - Response with value and metadata
   - `ErrorResponse` - Error response structure
   - `Metadata` - Timestamps (created_at, updated_at)

#### Should Have (P1) - Important

5. **Multiple UI Options**
   - Redoc UI at `/redoc` for read-focused browsing
   - Scalar UI at `/scalar` for modern, polished experience
   - RapiDoc UI at `/rapidoc` for minimal, fast interface
   - Allow users to choose their preferred documentation style

6. **API Metadata**
   - API title: "KV-Rust API"
   - API version: Match Cargo.toml version (0.1.0)
   - API description: Brief overview of the key-value store
   - Contact/license information (if applicable)

7. **Request Validation Examples**
   - Show key validation rules (alphanumeric, hyphens, underscores, 1-255 chars)
   - Document value constraints (non-empty string)
   - Example valid and invalid requests

8. **Response Examples**
   - Include example responses for success cases
   - Include example error responses
   - Show timestamp formats

#### Nice to Have (P2) - Future

9. **OpenAPI Spec as Static File**
   - Generate `openapi.json` as part of build process
   - Commit to repo for version control
   - Enable diffing spec changes in PRs

10. **Tags/Grouping**
    - Group endpoints by category (Health, Keys Read Ops, Keys Write Ops)
    - Improve navigation in documentation UI

11. **Server URL Configuration**
    - Allow configuring base URL via environment variable
    - Support multiple environments (dev, staging, prod)

12. **Custom Examples**
    - Provide realistic example keys/values
    - Show common use cases

### Non-Functional Requirements

#### Performance
- Documentation endpoints should not impact API performance
- Swagger UI assets served efficiently (static files, caching)
- OpenAPI spec generation happens at compile-time (zero runtime cost)

#### Maintainability
- Documentation derives from code (DRY principle)
- Compiler enforces documentation stays in sync
- Adding new endpoints automatically includes them in docs (with annotations)

#### Security
- Documentation endpoints available without authentication (public info)
- No sensitive data exposed in examples
- No security vulnerabilities in served static assets

#### Compatibility
- OpenAPI 3.0+ spec (widely supported)
- Works with modern browsers (Chrome, Firefox, Safari, Edge)
- Mobile-responsive documentation UIs

## Technical Considerations

### Technology Choice: `utoipa`

**Selected**: `utoipa` + `utoipa-actix-web` + `utoipa-swagger-ui`

**Why?**
- Most mature Rust OpenAPI solution (4.5k+ GitHub stars)
- Compile-time spec generation (type-safe, zero runtime cost)
- Excellent Actix-web v4 support
- Active maintenance and community
- Supports multiple documentation UIs
- Works seamlessly with Serde (already used in project)

**Alternatives considered**:
- `paperclip`: Less maintained, Actix v3 support only
- Manual OpenAPI spec: Too much maintenance burden, no type safety
- External tools (Postman, etc.): Not self-hosted, requires separate tools

**Related ADR**: Should create ADR-002 documenting this decision

### Implementation Approach

1. **Add Dependencies** (`Cargo.toml`)
   ```toml
   utoipa = { version = "5", features = ["actix_extras"] }
   utoipa-swagger-ui = { version = "8", features = ["actix-web"] }
   utoipa-redoc = { version = "5", features = ["actix-web"] }
   utoipa-rapidoc = { version = "5", features = ["actix-web"] }
   utoipa-scalar = { version = "1", features = ["actix-web"] }
   ```

2. **Annotate Models** (Add `#[derive(ToSchema)]`)
   - `CreateKVRequest`
   - `UpdateKVRequest`
   - `KeyValueResponse`
   - `ValueResponse`
   - `ErrorResponse`
   - `ErrorDetail`
   - `Metadata`

3. **Annotate Endpoints** (Add `#[utoipa::path(...)]`)
   - Describe each endpoint's behavior
   - Document parameters, request body, responses
   - Include status codes and descriptions
   - Add examples where helpful

4. **Create OpenAPI Struct** (`src/app/mod.rs` or new `src/app/docs.rs`)
   ```rust
   #[derive(OpenApi)]
   #[openapi(
       paths(
           health::health,
           read_ops::get_value_by_key,
           write_ops::create_kv,
           write_ops::get_keys_list,
           write_ops::update_kv,
           write_ops::delete_kv,
       ),
       components(schemas(
           CreateKVRequest,
           UpdateKVRequest,
           KeyValueResponse,
           ValueResponse,
           ErrorResponse,
           ErrorDetail,
           Metadata,
       )),
       info(
           title = "KV-Rust API",
           version = "0.1.0",
           description = "A lightweight key-value store REST API built with Rust and Actix-web"
       )
   )]
   struct ApiDoc;
   ```

5. **Mount Documentation UIs** (`src/main.rs`)
   ```rust
   use utoipa_swagger_ui::SwaggerUi;
   use utoipa_redoc::{Redoc, Servable};
   use utoipa_rapidoc::RapiDoc;
   use utoipa_scalar::{Scalar, Servable as ScalarServable};

   App::new()
       .service(
           SwaggerUi::new("/swagger-ui/{_:.*}")
               .url("/api-docs/openapi.json", ApiDoc::openapi())
       )
       .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
       .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
       .service(Scalar::with_url("/scalar", ApiDoc::openapi()))
       // ... existing routes
   ```

### Challenges & Risks

1. **Macro Annotations Verbosity**: May need many annotations, increasing code size
   - *Mitigation*: Keep annotations concise, use defaults where possible

2. **Compile Time Impact**: More macros may slow down compilation
   - *Mitigation*: Monitor build times, acceptable for better type safety

3. **Learning Curve**: Team needs to learn `utoipa` macro syntax
   - *Mitigation*: Document patterns, provide examples in first PR

4. **Breaking Changes**: `utoipa` API could change (though stable now)
   - *Mitigation*: Pin dependency versions, test before upgrading

5. **UI Asset Size**: Multiple UIs increase binary/deployment size
   - *Mitigation*: UIs are feature-gated, can disable unused ones later

### Dependencies

- Depends on existing API endpoints (all implemented)
- No blocking dependencies
- Independent feature, can be added without modifying business logic

## Design & User Experience

### URL Structure

| URL | Purpose |
|-----|---------|
| `/api-docs/openapi.json` | OpenAPI 3.0 spec (JSON) |
| `/swagger-ui` | Swagger UI (interactive testing) |
| `/redoc` | Redoc UI (clean reading experience) |
| `/scalar` | Scalar UI (modern, polished) |
| `/rapidoc` | RapiDoc UI (minimal, fast) |

### Example: Swagger UI Experience

1. User navigates to `http://localhost:8080/swagger-ui`
2. Sees list of all endpoints grouped by tags
3. Expands `GET /keys/{key}` endpoint
4. Sees:
   - Description: "Retrieve a value by its key"
   - Parameters: `key` (path, required, string, pattern: `^[a-zA-Z0-9_-]{1,255}$`)
   - Responses:
     - 200: Success with `ValueResponse` schema
     - 404: Key not found with `ErrorResponse` schema
   - "Try it out" button
5. Clicks "Try it out", enters `key = "user-123"`
6. Clicks "Execute"
7. Sees actual API response with data

### Example: OpenAPI Spec Output

```json
{
  "openapi": "3.0.0",
  "info": {
    "title": "KV-Rust API",
    "version": "0.1.0",
    "description": "A lightweight key-value store REST API built with Rust and Actix-web"
  },
  "paths": {
    "/keys/{key}": {
      "get": {
        "summary": "Get value by key",
        "parameters": [{
          "name": "key",
          "in": "path",
          "required": true,
          "schema": { "type": "string", "pattern": "^[a-zA-Z0-9_-]{1,255}$" }
        }],
        "responses": {
          "200": {
            "description": "Successfully retrieved value",
            "content": {
              "application/json": {
                "schema": { "$ref": "#/components/schemas/ValueResponse" }
              }
            }
          },
          "404": {
            "description": "Key not found",
            "content": {
              "application/json": {
                "schema": { "$ref": "#/components/schemas/ErrorResponse" }
              }
            }
          }
        }
      }
    }
  },
  "components": {
    "schemas": {
      "ValueResponse": {
        "type": "object",
        "required": ["value", "metadata"],
        "properties": {
          "value": { "type": "string" },
          "metadata": { "$ref": "#/components/schemas/Metadata" }
        }
      }
    }
  }
}
```

### Error Handling

Documentation itself has minimal error scenarios:
- Documentation endpoints always available (no auth required)
- Static assets served by framework (standard 404 if missing)
- OpenAPI spec generation happens at compile time (fails at build, not runtime)

## Implementation Plan

### Phase 1: Core Documentation (P0) - Est. 4-6 hours

1. **Add dependencies** to `Cargo.toml`
2. **Annotate models** with `#[derive(ToSchema)]`
3. **Annotate endpoints** with `#[utoipa::path(...)]`
4. **Create `ApiDoc` struct** with OpenAPI configuration
5. **Mount Swagger UI** at `/swagger-ui`
6. **Expose spec** at `/api-docs/openapi.json`
7. **Test all endpoints** appear in UI correctly
8. **Verify "Try it"** functionality works

**Deliverable**: Working Swagger UI with all endpoints documented

### Phase 2: Multiple UIs (P1) - Est. 2-3 hours

1. **Add UI dependencies** (redoc, scalar, rapidoc)
2. **Mount additional UIs** at respective paths
3. **Test each UI** renders correctly
4. **Add navigation links** between UIs (optional)

**Deliverable**: Four documentation UIs available

### Phase 3: Enhanced Documentation (P1-P2) - Est. 2-4 hours

1. **Add API metadata** (description, version, contact)
2. **Group endpoints by tags** (Health, Read Ops, Write Ops)
3. **Add detailed descriptions** to endpoints
4. **Include examples** for requests/responses
5. **Document validation rules** clearly

**Deliverable**: Polished, professional documentation

### Timeline

- **Week 1**: Phase 1 (core documentation)
- **Week 1-2**: Phase 2 (multiple UIs)
- **Week 2**: Phase 3 (enhancements)

**Total Estimated Effort**: 8-13 hours across 1-2 weeks

### Stacked PR Strategy

This feature can be implemented as stacked PRs:

1. **PR 1**: Add dependencies and annotate models (`#[derive(ToSchema)]`)
2. **PR 2**: Annotate endpoints with `#[utoipa::path(...)]`
3. **PR 3**: Mount Swagger UI and expose OpenAPI spec
4. **PR 4**: Add additional UIs (Redoc, Scalar, RapiDoc)
5. **PR 5**: Enhance with tags, descriptions, examples

Each PR is independently reviewable and testable.

## Open Questions

1. **Should we version the API in the URL** (e.g., `/v1/keys`)? 
   - Decision: Defer to future, current spec includes version in metadata

2. **Should documentation be available in production?**
   - Decision: Yes, unless security concerns arise (make it feature-gated if needed)

3. **Should we commit the generated OpenAPI spec to the repo?**
   - Decision: Not initially (P2), but consider for better visibility in PRs

4. **Should we add a landing page at `/` that links to documentation?**
   - Decision: Out of scope for this PRD, but good future enhancement

## Related Documentation

- **ADR-001**: Web API Endpoint Design (existing endpoints being documented)
- **ADR-002**: API Documentation Technology Choice (to be created)
- **Storage Service Layer Spec**: Documents the storage layer used by endpoints

## Success Criteria Checklist

- [ ] All REST endpoints appear in Swagger UI
- [ ] OpenAPI spec available at `/api-docs/openapi.json`
- [ ] "Try it" functionality works for all endpoints
- [ ] Request/response schemas documented
- [ ] Error responses included (404, 409, 400, 500)
- [ ] At least 2 documentation UIs available
- [ ] API version matches Cargo.toml
- [ ] All PRs include tests
- [ ] Documentation builds without warnings
- [ ] README updated with documentation URLs

## Appendix: Example Annotations

### Model Annotation
```rust
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateKVRequest {
    #[schema(example = "user-123")]
    pub key: Key,
    
    #[schema(example = "John Doe")]
    pub value: String,
}
```

### Endpoint Annotation
```rust
#[utoipa::path(
    get,
    path = "/keys/{key}",
    params(
        ("key" = String, Path, description = "Unique key identifier")
    ),
    responses(
        (status = 200, description = "Successfully retrieved value", body = ValueResponse),
        (status = 404, description = "Key not found", body = ErrorResponse)
    ),
    tag = "Keys"
)]
#[get("/keys/{key}")]
pub async fn get_value_by_key(...) { }
```

---

**Document Status**: Ready for review and approval
**Next Steps**: Create ADR-002 for technology choice, begin Phase 1 implementation
