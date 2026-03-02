# Matrix-First Strategy

Vauxl is pivoting to a Matrix-first architecture.

## Direction

- Vauxl provides a Matrix client implementing standard Matrix behavior first.
- Vauxl provides a Matrix server path (fork and/or implementation) focused on strict interoperability.
- Vauxl-specific features are implemented as optional extensions, fully documented and discoverable.

## Compatibility Rules

1. **Matrix baseline first**: all core user flows must work without Vauxl extensions.
2. **Open extensions only**: custom capabilities are public, versioned, and namespaced as `org.vauxl.*`.
3. **Graceful fallback**: clients or servers without extension support keep working with reduced functionality.
4. **Spec + implementation parity**: no production extension ships without docs and interoperability notes.

## Repositories

- Active: `docs`, `website`, `client`, `server`, `matrix-spec`
- Archived legacy repos are stored in `_archiv` with full `git bundle` backups.

## Initial MVP Scope

### Client MVP
- Account login/session handling
- Room list + timeline sync
- 1:1 and group messaging
- Media upload/download
- Baseline E2EE support

### Server MVP
- Compliant homeserver baseline
- Room/event/sync/media endpoints
- Federation baseline support
- Extension hook points and discovery
