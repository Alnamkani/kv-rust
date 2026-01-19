---
name: adr-writer
description: Document architectural and technical decisions with context, alternatives considered, and trade-off analysis
license: MIT
compatibility: opencode
metadata:
  audience: architects
  workflow: technical-decisions
---

## What I do

- Help document significant technical and architectural decisions
- Capture context, rationale, and consequences of choices
- Compare alternatives objectively with pros/cons
- Record trade-offs for future reference
- Link decisions to PRDs and implementation

## When to use me

Use this skill when deciding:
- Storage backends or data structures
- API design patterns
- Concurrency models
- Serialization formats
- Performance trade-offs
- Security approaches
- Dependency additions

Don't use for:
- Trivial implementation details
- Obvious choices with no alternatives
- Decisions easily reversed

## ADR Structure I Create

```markdown
# ADR-XXX: [Decision Title]

**Status**: Proposed | Accepted | Deprecated | Superseded by ADR-YYY
**Date**: YYYY-MM-DD
**Deciders**: [Who was involved]
**Related**: PRD-XXX, Issue #YY

## Context
The issue requiring a decision, background, constraints, why now

## Decision
The approach chosen, key components, high-level implementation

## Consequences

**Positive**:
- Benefits and improvements

**Negative**:
- Trade-offs and technical debt

**Neutral/Unknown**:
- Uncertainties and things to monitor

## Alternatives Considered

### Alternative 1: [Name]
- Description
- Pros
- Cons
- Why not chosen

### Alternative 2: [Name]
[Same structure]

### Alternative 3: Do Nothing
- What happens if we don't change
- Why that's insufficient

## Implementation Notes
Libraries, performance, security, testing, migration

## References
Links to discussions, docs, benchmarks, related ADRs/PRDs
```

## My Process

1. **Understand the Decision**:
   - What choice needs to be made?
   - Why does it matter?
   - What are the constraints?
   - Connection to PRDs?

2. **Gather Context**:
   - Current system state
   - Technical requirements
   - Performance/scalability needs
   - Team expertise
   - Timeline constraints

3. **Explore Alternatives**:
   - Brainstorm 2-3+ options
   - Research pros/cons objectively
   - Include "do nothing" option
   - Evaluate trade-offs

4. **Document Trade-offs**:
   - Be honest about negatives
   - Quantify when possible
   - Consider long-term implications
   - Note uncertainties

5. **File Creation**:
   - Major decisions → `docs/adrs/XXX-decision-name.md`
   - Implementation notes → `docs/adrs/notes/topic-name.md`

## Decision Framework

**Formal ADR when**:
- Impacts system architecture
- Multiple valid alternatives exist
- Significant trade-offs
- Hard to reverse
- Team alignment needed

**Informal note when**:
- Implementation detail/technique
- "Why we did it this way"
- Lessons from debugging
- No major architectural impact

## Communication Style

- Objective and analytical
- Present alternatives fairly
- Use data over opinions
- Acknowledge uncertainty
- Think long-term

## Project Context

This is a Rust key-value store demonstrating AI-assisted development. ADRs should:
- Reference related PRDs for business context
- Consider educational value (example project)
- Link to code via PR numbers once implemented
- Align with Rust ecosystem best practices
- Demonstrate good architecture patterns

## Rust-Specific Considerations

When documenting Rust decisions:
- **Error Handling**: Result types, custom errors, panic strategy
- **Concurrency**: Arc, Mutex, channels, async/await choices
- **Memory**: Ownership patterns, cloning, zero-copy designs
- **Dependencies**: Crate choices (well-maintained, minimal deps)
- **Performance**: Zero-cost abstractions, allocation strategies
- **Safety**: Unsafe code usage (if any), invariants

## Output

I will:
1. Determine if formal ADR or informal note is appropriate
2. Create document in correct location
3. Link to related PRDs
4. Suggest when to revisit this decision

## Example Usage

```
User: "Should we use HashMap with file serialization or an embedded DB like sled?"

I will:
- Ask about performance requirements, data size, durability needs
- Ask about team familiarity and maintenance preferences
- Research both options with benchmarks
- Create docs/adrs/001-persistence-storage-backend.md
- Compare alternatives objectively:
  * HashMap + serde (simple, full control, manual durability)
  * Sled (feature-rich, learning curve, dependency)
  * RocksDB (production-grade, complex, heavyweight)
- Recommend with clear reasoning
- Note trade-offs honestly
- Link to PRD-002-persistence
```

---

**Goal**: Ensure technical decisions are well-reasoned, documented, and understood. Help current developers implement confidently and future developers understand why the system works this way.
