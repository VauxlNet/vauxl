# Product Requirements Document

## Document Information

| Field | Value |
|-------|-------|
| Product Name | Vauxl |
| Version | 1.0 |
| Last Updated | 2026-01-18 |
| Status | Draft |

## Table of Contents

1. [Product Vision](#product-vision)
2. [Problem Statement](#problem-statement)
3. [Target Users](#target-users)
4. [Product Scope](#product-scope)
5. [Functional Requirements](#functional-requirements)
6. [Non-Functional Requirements](#non-functional-requirements)
7. [Platform Requirements](#platform-requirements)
8. [Security Requirements](#security-requirements)
9. [Success Metrics](#success-metrics)
10. [Constraints and Assumptions](#constraints-and-assumptions)
11. [Release Phases](#release-phases)
12. [Appendix: Glossary](#appendix-glossary)

## Product Vision

Vauxl is a secure, open-source communication platform that combines the community features of Discord with the security guarantees of Signal. It provides end-to-end encrypted text, voice, and video communication for groups ranging from two users to tens of thousands, without compromising on performance or user experience.

### Vision Statement

To deliver a communication platform where privacy is the default, large-scale community features are fully encrypted, and the user experience matches or exceeds commercial alternatives.

### Strategic Objectives

1. Demonstrate that strong security and rich features are not mutually exclusive.
2. Provide an open-source alternative to centralized, privacy-invasive platforms.
3. Enable communities to own their communication infrastructure.
4. Establish a new standard for secure group communication using MLS.

## Problem Statement

### Current Market Limitations

**Discord and Slack:**
- No end-to-end encryption for messages.
- Built on Electron, resulting in high memory usage (500MB+ idle).
- Centralized data storage accessible to the provider.
- Proprietary protocols limit interoperability.

**Signal:**
- End-to-end encryption, but limited group features.
- Double Ratchet protocol scales linearly O(N) for group operations.
- Practical group size limit around 1,000 members.
- No server/community structure.

**Matrix/Element:**
- Federated architecture complicates key management.
- Megolm encryption has known limitations for large groups.
- User experience lags behind commercial competitors.

### Opportunity

No existing platform combines:
- Enterprise-grade security (E2EE, forward secrecy)
- Large-scale community support (10,000+ member groups)
- Native performance (no Electron/WebView overhead)
- Open-source transparency

Vauxl addresses this gap.

## Target Users

### Primary User Segments

#### Privacy-Conscious Communities

- Open source project teams
- Privacy advocacy organizations
- Security researchers
- Whistleblower support networks

**Needs:** Strong encryption, open source auditability, no metadata collection.

#### Enterprise and Professional Teams

- Remote engineering teams
- Legal and compliance-sensitive organizations
- Healthcare communication (HIPAA considerations)
- Financial services (regulatory compliance)

**Needs:** Audit logs, administrative controls, compliance certifications.

#### Content Creator Communities

- Gaming communities
- Educational platforms
- Fan communities
- Creative collaboration groups

**Needs:** Rich media support, voice channels, moderation tools, scalability.

### User Personas

**Persona 1: Security-Focused Developer**
- Maintainer of popular open-source project
- Currently uses Discord for community, concerned about privacy
- Wants E2EE without losing community interaction features
- Technical, comfortable with self-hosting

**Persona 2: Community Manager**
- Manages 5,000+ member gaming community
- Needs moderation tools, role management
- Requires mobile and desktop access
- Less technical, expects polished UX

**Persona 3: Compliance Officer**
- Works at financial services firm
- Needs encrypted communication for sensitive discussions
- Requires audit trails and data retention controls
- Evaluates tools against regulatory requirements

## Product Scope

### In Scope (Version 1.0)

- Encrypted text messaging (1:1 and group)
- Encrypted voice channels
- Server and channel structure (Discord-like hierarchy)
- Role-based permissions
- Cross-platform clients (Windows, Linux, Android)
- Self-hosted server option
- Basic moderation tools

### Out of Scope (Version 1.0)

- Video channels (planned for v1.1)
- Screen sharing (planned for v1.1)
- iOS client (planned for v1.2)
- macOS client (planned for v1.2)
- Web client (under evaluation)
- Federated servers (under research)
- Bots and integrations API (planned for v2.0)
- Marketplace for themes/plugins (not planned)

## Functional Requirements

### FR-100: User Management

#### FR-101: Account Registration

Users shall be able to create accounts with:
- Username (unique, 3-32 characters, alphanumeric with underscores)
- Password (minimum 12 characters, complexity requirements)
- Email (optional, for recovery)

Account creation shall generate local cryptographic identity keys.

#### FR-102: Authentication

Users shall authenticate using:
- Password-based login
- Device-specific session tokens
- Optional two-factor authentication (TOTP)

#### FR-103: Profile Management

Users shall be able to:
- Set display name (separate from username)
- Upload avatar image
- Set status message
- Configure presence visibility

#### FR-104: Account Recovery

If email is provided, users shall be able to:
- Request password reset via email
- Reset password and re-establish encryption keys

Warning: Password reset invalidates existing encryption keys; message history will be inaccessible.

### FR-200: Messaging

#### FR-201: Direct Messages

Users shall be able to:
- Send text messages to other users
- Send rich text (Markdown formatting)
- Send images and files (up to 100MB each)
- Edit sent messages (within 15 minutes)
- Delete sent messages
- View read receipts (configurable)

All messages shall be end-to-end encrypted.

#### FR-202: Group Messages

Servers shall contain channels where:
- Members with permission can send messages
- Message history is persisted locally
- Messages are encrypted for the group

#### FR-203: Message Formatting

Supported formatting:
- Bold, italic, underline, strikethrough
- Code blocks (inline and multi-line with syntax highlighting)
- Blockquotes
- Bulleted and numbered lists
- Links (with preview)
- Mentions (@user, @role, @everyone, @here)

#### FR-204: Reactions

Users shall be able to:
- Add emoji reactions to messages
- Remove their own reactions
- View reaction counts and who reacted

#### FR-205: Search

Users shall be able to search:
- Within a channel
- Across a server (if permitted)
- In direct messages

Search executes locally on decrypted messages.

### FR-300: Voice Communication

#### FR-301: Voice Channels

Servers shall support voice channels where:
- Members can join and speak
- No persistent history (real-time only)
- Audio is end-to-end encrypted

#### FR-302: Voice Controls

Users in voice channels can:
- Mute/unmute microphone
- Adjust input/output volume
- Toggle noise suppression
- Push-to-talk mode (optional)

#### FR-303: Voice Channel Permissions

Configurable permissions:
- Speak
- Connect
- Priority speaker
- Move members

### FR-400: Servers and Channels

#### FR-401: Server Creation

Users shall be able to:
- Create servers with a name and icon
- Receive owner role with full permissions
- Invite others via invite links

#### FR-402: Channel Types

- Text channels: Persistent message history
- Voice channels: Real-time audio
- Announcement channels: Read-only for most users

#### FR-403: Channel Organization

Channels can be organized into:
- Categories (collapsible groups)
- Ordered within categories

#### FR-404: Invite System

- Invite links with optional expiration
- Invite links with optional use limit
- Revocable by moderators

### FR-500: Roles and Permissions

#### FR-501: Role Definition

Roles have:
- Name and color
- Position in hierarchy
- Permission set

#### FR-502: Permission Categories

| Category | Permissions |
|----------|-------------|
| General | View channels, Manage channels, Manage server |
| Membership | Ban members, Kick members, Manage roles |
| Text | Send messages, Manage messages, Mention everyone |
| Voice | Connect, Speak, Mute members, Deafen members |

#### FR-503: Permission Resolution

Permissions are resolved by:
1. Server-level role permissions
2. Channel-specific role overrides
3. Channel-specific user overrides

Deny overrides grant.

### FR-600: Moderation

#### FR-601: Moderation Actions

Moderators can:
- Delete messages
- Kick members (can rejoin)
- Ban members (cannot rejoin)
- Timeout members (temporary mute)

#### FR-602: Ban Management

- Ban with optional reason
- Ban list viewable by moderators
- Unban capability

#### FR-603: Audit Log

Server audit log records:
- Moderation actions
- Role changes
- Channel changes
- Server setting changes

Audit log entries include actor, action, target, and timestamp.

### FR-700: Notifications

#### FR-701: Notification Types

- New direct message
- Mention in channel
- Server announcement
- Voice channel activity (user joined)

#### FR-702: Notification Settings

Configurable per server/channel:
- All messages
- Mentions only
- None
- Override for @everyone and @role mentions

#### FR-703: Push Notifications

Mobile devices receive push notifications when:
- App is in background
- Notification settings allow

Push content is encrypted; server cannot read notification content.

## Non-Functional Requirements

### NFR-100: Performance

#### NFR-101: Application Startup

- Cold start to interactive: Under 3 seconds on reference hardware
- Reference hardware: Mid-range Android phone (2023), mid-range desktop

#### NFR-102: Message Send Latency

- Local processing (encrypt, queue): Under 50ms
- Round-trip to server and back: Under 200ms (excluding network)

#### NFR-103: Voice Latency

- Audio capture to remote playback: Under 300ms
- Jitter: Under 30ms

#### NFR-104: Scroll Performance

- Chat history scrolling: 60fps with 100,000 messages
- Achieved via virtual list rendering

#### NFR-105: Memory Usage

- Idle state: Under 150MB RAM
- Active voice channel: Under 300MB RAM

### NFR-200: Reliability

#### NFR-201: Availability

- Client: Functional offline (read cached messages, queue outgoing)
- Server: 99.9% uptime target for hosted deployments

#### NFR-202: Message Delivery

- Messages are queued when offline
- Delivery confirmed via receipt system
- No message loss due to transient connectivity

#### NFR-203: Crash Recovery

- Application state persisted regularly
- Unsent messages recovered after crash
- Cryptographic state survives crash

### NFR-300: Scalability

#### NFR-301: Group Size

- Text channels: 50,000+ members
- Voice channels: 100+ simultaneous speakers (with SFU)

#### NFR-302: Server Load

- Single API server: 10,000 concurrent WebSocket connections
- Horizontal scaling supported

### NFR-400: Usability

#### NFR-401: Accessibility

- Screen reader compatibility
- Keyboard navigation
- Color contrast compliance (WCAG AA)

#### NFR-402: Internationalization

- UTF-8 throughout
- RTL language support
- Localization framework (translations in future versions)

### NFR-500: Maintainability

#### NFR-501: Code Quality

- Automated testing: 80% code coverage minimum
- Static analysis: No warnings
- Formatted code: Enforced by CI

#### NFR-502: Documentation

- Public API documented
- Architectural decisions recorded
- Deployment guides provided

## Platform Requirements

### PR-100: Client Platforms

#### PR-101: Windows

- Windows 10 version 1809 or later
- x86_64 architecture
- Native executable (no runtime dependencies)

#### PR-102: Linux

- Ubuntu 20.04 LTS or equivalent
- x86_64 architecture
- GTK3 or compatible display server

#### PR-103: Android

- Android 8.0 (API 26) or later
- ARM64 and x86_64 architectures
- Play Store and APK distribution

### PR-200: Server Platforms

#### PR-201: API Server

- Linux (Debian, Ubuntu, or compatible)
- Docker container available
- Kubernetes deployment manifests provided

#### PR-202: Media Server

- LiveKit server requirements
- Linux recommended
- Sufficient bandwidth for expected concurrency

#### PR-203: Database

- PostgreSQL 15 or later
- Minimum 10GB storage (scales with usage)

## Security Requirements

### SR-100: Encryption

#### SR-101: End-to-End Encryption

All user message content shall be encrypted end-to-end using MLS (RFC 9420).

- Server cannot decrypt messages
- Forward secrecy via epoch rotation
- Post-compromise security

#### SR-102: Data at Rest

Local storage shall be encrypted:

- SQLite encrypted with sqlcipher
- Key derived from user password
- Separate encryption key for each database

#### SR-103: Data in Transit

All network communication shall use:

- TLS 1.3 for WebSocket connections
- DTLS for media streams
- Certificate pinning recommended for high-security deployments

### SR-200: Authentication

#### SR-201: Password Security

- Argon2id for password hashing
- Minimum 12 characters
- No known breached password check

#### SR-202: Session Management

- Sessions bound to device
- Session revocation capability
- Automatic expiration after inactivity (configurable)

### SR-300: Key Management

#### SR-301: Key Generation

- Cryptographic keys generated using system CSPRNG
- Ed25519 for identity keys
- X25519 for key agreement

#### SR-302: Key Storage

- Private keys never leave device
- Keys encrypted at rest
- Key backup optional (encrypted export)

### SR-400: Auditing

#### SR-401: Security Logging

Server logs (without message content):

- Authentication attempts
- Key package uploads
- Group membership changes
- Administrative actions

#### SR-402: Client Transparency

Users can view:

- Active sessions
- Key fingerprints
- Group member keys

## Success Metrics

### Adoption Metrics

| Metric | Target (12 months post-launch) |
|--------|-------------------------------|
| Registered users | 50,000 |
| Daily active users | 10,000 |
| Monthly active servers | 1,000 |
| Messages sent (monthly) | 10,000,000 |

### Quality Metrics

| Metric | Target |
|--------|--------|
| Crash-free sessions | 99.5% |
| App store rating | 4.0+ |
| Support tickets (per 1000 users) | Under 10 monthly |

### Security Metrics

| Metric | Target |
|--------|--------|
| Critical vulnerabilities (unpatched) | 0 |
| Time to patch critical issues | Under 48 hours |
| Security audit findings (critical) | 0 at launch |

## Constraints and Assumptions

### Technical Constraints

1. No JavaScript in client (violates type safety requirement)
2. No Java in client core (memory safety requirement)
3. No C/C++ source code (except pre-built libraries like libwebrtc via LiveKit)
4. Rust for all business logic
5. Flutter for all UI code

### Business Constraints

1. Open source (license to be determined)
2. Self-hostable with reasonable complexity
3. No telemetry without explicit user consent

### Assumptions

1. Users accept initial key generation delay (first launch).
2. Users understand password loss may result in message history loss.
3. Target devices have reliable internet for initial sync.
4. MLS implementations (OpenMLS) are production-ready.

## Release Phases

### Phase 1: Foundation (Months 1-4)

**Deliverables:**
- Rust core with state management
- Flutter UI shell
- flutter_rust_bridge integration
- Local SQLite storage
- Basic text messaging (1:1 only)

**Exit Criteria:**
- Two users can exchange encrypted text messages
- Messages persist across app restarts

### Phase 2: Core Messaging (Months 5-8)

**Deliverables:**
- MLS group encryption
- Server and channel structure
- Roles and permissions
- Message formatting
- File attachments

**Exit Criteria:**
- 100-member group with encrypted messaging
- Role-based permissions enforced

### Phase 3: Voice (Months 9-12)

**Deliverables:**
- LiveKit integration
- Voice channels
- Audio processing (AEC, noise suppression)
- Push notifications

**Exit Criteria:**
- 20-user voice channel functional
- Sub-300ms audio latency

### Phase 4: Polish and Launch (Months 13-16)

**Deliverables:**
- Moderation tools
- Audit logging
- Performance optimization
- Security audit remediation
- Documentation

**Exit Criteria:**
- Security audit passed
- Performance targets met
- Deployment guides complete

## Appendix: Glossary

| Term | Definition |
|------|------------|
| AEC | Acoustic Echo Cancellation; removes speaker audio from microphone input |
| E2EE | End-to-End Encryption; only sender and recipients can decrypt content |
| FFI | Foreign Function Interface; calling code between languages |
| MLS | Messaging Layer Security; IETF RFC 9420 for group encryption |
| SFU | Selective Forwarding Unit; media server that routes streams |
| TreeKEM | Tree-based Key Encapsulation Mechanism; MLS key agreement |
| TURN | Traversal Using Relays around NAT; relay server for restricted networks |
| WSS | WebSocket Secure; encrypted WebSocket connection |
