# AI Agent Instructions for KV-Rust

This document defines how AI agents should work on the kv-rust codebase. These guidelines ensure consistency, quality, and alignment with the project's goals of demonstrating modern AI-assisted product development.

## Available Skills

This project provides specialized skills for different aspects of the development workflow:

- **prd-writer** (`.opencode/skills/prd-writer/SKILL.md`) - Creates Product Requirements Documents for new features
- **adr-writer** (`.opencode/skills/adr-writer/SKILL.md`) - Documents architectural and technical decisions

To use these skills, load them with the skill tool when planning or documenting features.

## Project Context

**Purpose**: This is a Rust-based key-value store serving as an educational example of how to build products in an AI-enabled startup environment.

**Key Principles**:
- Every feature has documented reasoning (PRDs, ADRs)
- Development uses stacked PRs via Graphite
- Code quality and clarity over cleverness
- Full traceability from idea to implementation

## Coding Standards

### Language & Style

- **No comments in code** - Code should be self-documenting through clear naming and structure
- **No dataclass** - Use Pydantic for data structures (when applicable)
- **Never use `typing.Optional`** - Use the union operator `|` instead (e.g., `str | None`)
- **Import organization** - Always import at the top of the file

### Rust-Specific Guidelines

- Follow idiomatic Rust patterns
- Use `Result<T, E>` for error handling, never panic in library code
- Prefer iterators over explicit loops where appropriate
- Use `clippy` recommendations (`cargo clippy`)
- Format code with `rustfmt` (`cargo fmt`)
- Write tests alongside implementation
- Document public APIs with doc comments (`///`)

### Code Structure

```rust
pub fn get_value(key: &str) -> Result<Option<String>, StorageError> {
    storage.retrieve(key)
}
```

Good: Clear, self-documenting names and structure

## Development Workflow

### Before Starting Implementation

1. **Sync with remote**: Always start by running `gt sync` to pull trunk, clean up merged branches, and restack
   - **NEVER use `git pull`** - always use `gt sync` instead
   - This ensures your local repository is up to date with the remote

2. **Check documentation**:
   - Is there a PRD for this feature? (`docs/prds/`)
   - Are there related ADRs? (`docs/adrs/`)
   - If not, use the **prd-writer** and **adr-writer** skills to create them

3. **Understand the stack**:
   - What features does this depend on?
   - What features might depend on this?
   - Should this be part of a stacked PR?

### During Implementation

1. **Atomic commits**:
   - Each commit should represent one logical change
   - Follow conventional commits format: `feat:`, `fix:`, `refactor:`, etc.
   - Reference docs in commit messages: `feat: add persistence layer (PRD-002, ADR-001)`

2. **Test coverage**:
   - Write unit tests for all new functionality
   - Add integration tests for feature workflows
   - Ensure `cargo test` passes before committing

3. **Documentation**:
   - Update ADRs if implementation differs from plan
   - Add notes to `docs/adrs/notes/` for interesting implementation details
   - Keep README updated with new features

### Creating Pull Requests

1. **PR Title**: Follow conventional commits format
   ```
   feat: add key-value persistence layer
   fix: handle edge case in get operation
   refactor: simplify storage interface
   ```

2. **PR Description Template**:
   ```markdown
   ## Related Documentation
   - PRD: docs/prds/XXX-feature-name.md
   - ADR: docs/adrs/XXX-decision-name.md
   
   ## Summary
   Brief description of what this PR does and why.
   
   ## Changes
   - Change 1
   - Change 2
   
   ## Testing
   - Test scenario 1
   - Test scenario 2
   
   ## Stack Info
   - Builds on: PR #XX (if stacked)
   - Enables: PR #YY (if base for others)
   ```

3. **Traceability**:
   - Link to relevant PRDs/ADRs
   - Reference related issues
   - Note if part of a Graphite stack

### Using Graphite for Stacked PRs

1. **Branch naming**: Use descriptive names with prefixes
   ```bash
   feat/kv-basic-ops
   feat/kv-persistence
   fix/error-handling
   refactor/storage-interface
   ```

2. **Creating stacks**:
   - Create a new branch with changes: `gt create -am "commit message"` (or `gt c -am "message"`)
   - This stages all changes and creates a commit in one command
   - Each branch should build on the previous when there's a dependency
   - View your stack: `gt log` or `gt ls` (short form)

3. **Submitting stacks**:
   - Submit current branch and ancestors: `gt submit`
   - Submit entire stack (current + descendants): `gt submit --stack` (or `gt ss`)
   - Publish PRs immediately: `gt submit --stack --publish` (or `gt ss -p`)
   - Always use `--publish` to make PRs public when submitting

4. **Updating stacks**:
   - Modify current branch: `gt modify` (or `gt m`)
   - Stage all and amend: `gt modify -a` (or `gt m -a`)
   - After changes, resubmit: `gt submit --stack --publish`
   - Sync with remote: `gt sync` (pulls trunk, cleans merged branches, restacks)

5. **Common Graphite Commands**:
   ```bash
   gt create -am "message"     # Create branch with commit (short: gt c -am)
   gt modify -a                # Amend changes to current branch (short: gt m -a)
   gt submit --stack --publish # Submit and publish stack (short: gt ss -p)
   gt sync                     # Sync with remote and clean up
   gt log                      # View your stacks (short: gt ls)
   gt up / gt down             # Navigate between branches (short: gt u / gt d)
   gt checkout <branch>        # Switch branches (short: gt co)
   gt move --onto <branch>     # Move current branch to new parent
   ```

## Decision Documentation

### When to Write an ADR

Create a formal ADR (`docs/adrs/XXX-name.md`) for:
- Choice of storage backend or data structures
- API design decisions
- Performance trade-offs
- Security considerations
- Dependency additions

Use the **adr-writer** skill to create these.

### When to Write a Note

Create an informal note (`docs/adrs/notes/name.md`) for:
- Implementation techniques or patterns used
- Interesting bugs and their solutions
- Alternative approaches considered but not chosen
- Learning points during development

## Code Review Guidelines

When reviewing code:

1. **Verify documentation**:
   - PR references appropriate PRD/ADR
   - Implementation matches documented decisions

2. **Check code quality**:
   - No unnecessary comments
   - Clear, self-documenting names
   - Proper error handling
   - Tests included

3. **Validate workflow**:
   - Commit messages follow conventions
   - Changes are atomic and logical
   - Stack structure makes sense

4. **Security & Performance**:
   - No hardcoded secrets or credentials
   - Efficient algorithms and data structures
   - Proper resource cleanup

## Example Session Flow

```
User: "Add a persistence layer to the KV store"

Agent: 
1. Sync with remote: gt sync
2. Load prd-writer skill to create PRD-002-persistence.md
3. Load adr-writer skill to create ADR-001-storage-backend-choice.md
4. Implement persistence layer with tests
5. Create Graphite branch with changes: gt create -am "feat: add persistence layer (PRD-002, ADR-001)"
6. Submit and publish PR: gt submit --stack --publish (or gt ss -p)
7. Update docs/adrs/notes/ with implementation insights
```

## Project-Specific Context

### Current Architecture
- Simple in-memory HashMap-based storage (as of initial implementation)
- Basic get/set/delete operations
- No persistence yet (upcoming in PRD-002)

### Planned Features
(This section should be updated as PRDs are created)

### Technology Stack
- **Language**: Rust (latest stable)
- **Build Tool**: Cargo
- **Testing**: Built-in Rust test framework
- **CI/CD**: (To be determined)
- **Version Control**: Git + Graphite for stacked PRs

## Tools to Use

- **Find files**: `fd` (not `find`)
- **Search text**: `rg` (ripgrep, not `grep`)
- **Find code structure**: `ast-grep` (set `--lang rust`)
- **Select among matches**: pipe to `fzf`
- **JSON processing**: `jq`

## Questions to Ask

Before implementing, AI agents should consider:

1. Is there a PRD for this feature?
2. Are there architectural decisions to document?
3. Does this depend on other features (stack it)?
4. Are there security implications?
5. How should this be tested?
6. What's the rollback plan if this doesn't work?

## Continuous Improvement

This document should evolve as the project grows. When patterns emerge or lessons are learned:

1. Update this document
2. Create an ADR note explaining the change
3. Ensure consistency across existing code

---

**Remember**: The goal is not just to build a KV store, but to demonstrate how AI can accelerate product development while maintaining high standards of documentation, code quality, and traceability.
