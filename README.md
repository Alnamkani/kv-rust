# KV-Rust

A Rust-based key-value store built as an example of modern AI-assisted product development in a startup environment.

## Purpose

This project demonstrates how to develop a production-grade application using AI assistance, with emphasis on:

- Clear documentation and decision tracking
- Structured product development process
- Transparent implementation reasoning
- Best practices for AI-augmented development

## Project Structure

```
kv-rust/
├── .opencode/              # OpenCode configuration
│   └── skills/             # Reusable AI skills
│       ├── prd-writer/     # PRD creation skill
│       └── adr-writer/     # ADR documentation skill
├── docs/                   # All documentation
│   ├── prds/               # Product Requirements Documents
│   │   └── specs/          # Lightweight feature specs
│   ├── adrs/               # Architecture Decision Records
│   │   └── notes/          # Informal implementation notes
│   └── process/            # Development workflow docs
├── src/                    # Source code
├── AGENTS.md               # AI agent instructions
└── README.md
```

## Documentation Philosophy

Every feature in this project follows a documented lifecycle:

1. **PRD (Product Requirement Document)** - Defines what we're building and why
2. **ADR (Architecture Decision Record)** - Documents key technical decisions
3. **Implementation** - Code with clear commit messages linking to docs
4. **Traceability** - PRs reference PRDs/ADRs for full audit trail

### Documentation Types

- **Major Features** → Full PRD in `docs/prds/` (numbered, e.g., `001-feature-name.md`)
- **Minor Features** → Lightweight spec in `docs/prds/specs/`
- **Architectural Decisions** → Formal ADR in `docs/adrs/` (numbered)
- **Implementation Notes** → Informal notes in `docs/adrs/notes/`

## Development Workflow with Stacked PRs (Graphite)

This project uses [Graphite](https://graphite.dev) for managing stacked pull requests, allowing us to work on dependent features in parallel without waiting for PR reviews.

### Why Stacked PRs?

Traditional workflow:
```
Feature A (PR #1) → wait for review → merge → Feature B (PR #2) → wait for review → merge
```

Stacked workflow:
```
Feature A (PR #1) ──┐
                    ├→ both in review simultaneously
Feature B (PR #2) ──┘  (B builds on A)
```

### Setup Graphite

1. Install Graphite CLI:
```bash
brew install withgraphite/tap/graphite
```

2. Authenticate with GitHub:
```bash
gt auth
```

3. Initialize Graphite in the repo:
```bash
gt repo init
```

### Working with Stacked PRs

#### Creating a Stack

1. **Start with main branch:**
```bash
git checkout main
git pull
```

2. **Create first feature branch:**
```bash
gt branch create "feature-a"
# Make changes, commit
git add .
git commit -m "feat: implement feature A (PRD-001)"
```

3. **Create dependent feature (stacked on top):**
```bash
gt branch create "feature-b"
# Make changes that depend on feature-a
git add .
git commit -m "feat: implement feature B (PRD-002)"
```

4. **Submit the stack:**
```bash
gt stack submit
```

This creates PR #1 (feature-a → main) and PR #2 (feature-b → feature-a)

#### Updating a Stack

When feature-a needs changes after review:

```bash
gt checkout feature-a
# Make changes
git add .
git commit -m "fix: address review comments"
gt stack submit  # Updates PR #1 and rebases PR #2 automatically
```

#### Merging a Stack

When feature-a is approved:

```bash
gt stack submit --merge
```

Graphite automatically:
- Merges feature-a to main
- Rebases feature-b onto main
- Updates PR #2 to point to main

### Useful Graphite Commands

```bash
gt log              # Visual representation of your stack
gt stack submit     # Submit/update all PRs in the stack
gt upstack onto     # Rebase current branch + upstack onto another branch
gt downstack get    # Pull latest changes from branches below
gt repo sync        # Sync with remote and clean up merged branches
```

### Best Practices for Stacked Development

1. **Keep changes atomic** - Each branch should represent one logical change
2. **Link to docs** - Every PR description references the relevant PRD/ADR
3. **Clear commit messages** - Follow conventional commits format
4. **Small stacks** - Limit stacks to 2-4 PRs for easier review
5. **Independent when possible** - Only stack when there's a true dependency

### Example Stack Workflow

```bash
# PRD-001: Basic key-value operations
gt branch create "feat/kv-basic-ops"
# Implement get/set operations
git commit -m "feat: add basic get/set operations (PRD-001)"

# PRD-002: Persistence (depends on basic ops)
gt branch create "feat/kv-persistence"
# Implement persistence layer
git commit -m "feat: add persistence layer (PRD-002, ADR-001)"

# PRD-003: Transaction support (depends on persistence)
gt branch create "feat/kv-transactions"
# Implement transactions
git commit -m "feat: add transaction support (PRD-003)"

# Submit all three PRs as a stack
gt stack submit
```

## AI Skills & Development Workflow

This project uses OpenCode skills to streamline the development process:

### Available Skills

- **prd-writer** (`.opencode/skills/prd-writer/SKILL.md`) - Creates comprehensive Product Requirements Documents
- **adr-writer** (`.opencode/skills/adr-writer/SKILL.md`) - Documents architectural decisions and technical trade-offs

### Using Skills

The typical feature development flow:

1. Load **prd-writer** skill to create a PRD for your feature
2. Load **adr-writer** skill to document key technical decisions
3. Implement the feature following the documented requirements
4. Create stacked PRs with Graphite linking to PRDs/ADRs

See [AGENTS.md](AGENTS.md) for detailed guidelines on how AI should work on this codebase.

## Getting Started

### Prerequisites

- Rust (install via `brew install rust`)
- Graphite CLI (install via `brew install withgraphite/tap/graphite`)
- Git

### Build and Run

```bash
cargo build
cargo run
```

### Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_health
```

See [docs/TESTING.md](docs/TESTING.md) for comprehensive testing guide.

## Contributing

All contributions should follow the documented workflow:

1. Check if a PRD exists for your feature, or create one
2. Document architectural decisions in ADRs
3. Use stacked PRs for dependent features
4. Link PRs to relevant documentation
5. Follow commit message conventions

## License

MIT


Phase 1: Health Endpoint (Current)
- ✅ Test GET /health returns 200
- ✅ Test GET /health returns "OK"
- ✅ Test wrong HTTP method returns 405
Phase 2: Service Layer
- [ ] Test creating new store
- [ ] Test setting a key-value pair
- [ ] Test getting existing key
- [ ] Test getting missing key
- [ ] Test overwriting existing key
- [ ] Test deleting existing key
- [ ] Test deleting missing key
- [ ] Test thread safety (Arc/Mutex)
Phase 3: Read Operations
- [ ] Test GET /keys/{key} with existing key returns 200
- [ ] Test GET /keys/{key} with missing key returns 404
- [ ] Test response JSON format
- [ ] Test special characters in keys
- [ ] Test empty string as key
Phase 4: Write Operations
- [ ] Test POST /keys creates new key
- [ ] Test POST /keys returns 201
- [ ] Test POST /keys with invalid JSON returns 400
- [ ] Test DELETE /keys/{key} removes existing key
- [ ] Test DELETE /keys/{key} returns 204 on success
- [ ] Test DELETE /keys/{key} returns 404 for missing key
Phase 5: Integration Tests
- [ ] Test full CRUD workflow
- [ ] Test concurrent requests
- [ ] Test edge cases (very long keys/values)
- [ ] Test error handling
