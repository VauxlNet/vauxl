# Protocol Agnostic Architecture Rule

This is a project architecture guideline, not an agent behavior policy.

## Goal

Design new features so protocol choice can evolve without rewriting core product logic.

## Required Pattern

- Keep domain logic independent from wire protocol details.
- Introduce protocol adapters at boundaries.
- Use capability negotiation, not protocol-specific assumptions in core logic.
- Map protocol events into a canonical internal model.
- Keep extension behavior optional with documented fallback.

## Practical Guidance

- Define stable internal interfaces for messaging, identity, presence, media, and sync.
- Place protocol-specific code behind adapter modules.
- Avoid leaking protocol event names and payload shapes into business logic.
- Write conformance tests per protocol adapter and behavior tests against the shared domain layer.

## Validation Checklist

- Can the domain layer run without importing protocol-specific packages?
- Can a second protocol adapter be added without editing core feature logic?
- Are unsupported capabilities handled through explicit fallback behavior?
- Are protocol conformance tests isolated from domain behavior tests?
