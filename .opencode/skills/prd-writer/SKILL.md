---
name: prd-writer
description: Create Product Requirements Documents for new features with structured requirements, success metrics, and implementation plans
license: MIT
compatibility: opencode
metadata:
  audience: product-managers
  workflow: feature-planning
---

## What I do

- Guide you through creating comprehensive PRDs with proper structure
- Ask clarifying questions about user needs, goals, and success metrics
- Translate business requirements into technical specifications
- Separate must-have from nice-to-have requirements
- Link PRDs to architectural decisions (ADRs)
- Determine appropriate numbering and file location

## When to use me

Use this skill when:
- Starting a new feature or capability
- Planning work that requires multiple engineering days
- Needing stakeholder alignment on requirements
- Feature has cross-cutting concerns or dependencies

Don't use for:
- Tiny bug fixes or tweaks
- Changes with obvious requirements
- Single-file, < 1 day changes

## PRD Structure I Create

```markdown
# PRD-XXX: [Feature Name]

**Status**: Draft | In Review | Approved | Implemented
**Owner**: [Stakeholder]
**Created**: YYYY-MM-DD

## Overview
Brief summary of the feature and its value

## Problem Statement
What problem we're solving and who experiences it

## Goals & Success Metrics
- Goals (what we want to achieve)
- Success Metrics (how we measure)
- Non-Goals (scope boundaries)

## User Stories
As a [user], I want [action] so that [benefit]

## Requirements

### Functional Requirements
- Must Have (P0) - Critical
- Should Have (P1) - Important
- Nice to Have (P2) - Future

### Non-Functional Requirements
- Performance, Scalability, Security, Reliability, Compatibility

## Technical Considerations
High-level approach, challenges, dependencies, risks

## Design & User Experience
API examples, CLI mockups, error handling

## Implementation Plan
Phases, timeline, dependencies

## Open Questions
Items needing resolution

## Related Documentation
Links to ADRs, other PRDs, issues
```

## My Process

1. **Discovery**: Ask questions about problem, users, success, constraints
2. **Structure**: Organize into PRD format with concrete examples
3. **Technical Translation**: Convert business needs to technical requirements
4. **Validation**: Ensure completeness, measurability, clear scope
5. **File Creation**:
   - Major features → `docs/prds/XXX-feature-name.md`
   - Minor features → `docs/prds/specs/feature-name.md`

## Decision Framework

**Full PRD when**:
- Multi-day engineering effort
- Cross-cutting concerns
- Multiple stakeholders
- Architectural decisions needed

**Lightweight spec when**:
- Small isolated change
- Clear requirements
- < 1 day effort
- No architectural impact

## Project Context

This is a Rust key-value store demonstrating AI-assisted product development. PRDs should:
- Link to relevant ADRs for technical decisions
- Fit into stacked PR workflow with Graphite
- Consider educational/documentation value
- Maintain traceability to code via PR references

## Output

I will:
1. Ask clarifying questions to understand the feature
2. Create the PRD document in the appropriate location
3. Suggest related ADRs if architectural decisions are needed
4. Provide next steps for implementation

## Example Usage

```
User: "I want to add persistence to the KV store so data survives restarts"

I will:
- Ask about performance requirements, data size, durability needs
- Ask about target use cases and users
- Create docs/prds/002-persistence-layer.md
- Suggest creating ADR for storage backend choice
- Link to implementation plan with stacked PRs
```

---

**Goal**: Ensure everyone understands what's being built, why it matters, and what success looks like before any code is written.
