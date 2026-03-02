# Service Model and Deployment Architecture

This document describes the Vauxl service offerings, deployment models, licensing structure, and infrastructure operated by the Vauxl team.

## Table of Contents

1. [Overview](#overview)
2. [Vauxl-Operated Infrastructure](#vauxl-operated-infrastructure)
3. [Deployment Models](#deployment-models)
4. [Server Architecture](#server-architecture)
5. [Licensing Model](#licensing-model)
6. [Enterprise Offerings](#enterprise-offerings)
7. [Offline and LAN Deployment](#offline-and-lan-deployment)
8. [Account and Identity System](#account-and-identity-system)

## Overview

Vauxl is designed with flexibility at its core. Users can choose between fully managed services hosted by the Vauxl team, self-hosted deployments with varying levels of connectivity, or completely offline installations for air-gapped environments.

### Guiding Principles

1. **Freedom to Self-Host**: Any individual or organization can deploy their own Vauxl server with minimal effort.
2. **Zero Friction Start**: The hosted free tier allows immediate use without infrastructure knowledge.
3. **Enterprise Ready**: Organizations requiring compliance, data sovereignty, or isolation have clear paths.
4. **Offline Capable**: The platform functions in disconnected environments (LAN parties, secure facilities).

## Vauxl-Operated Infrastructure

The Vauxl team operates core infrastructure to provide baseline services for all users.

### Central Services

| Service | Purpose | Availability |
|---------|---------|--------------|
| Auth Server | Identity management, cross-device sync | Always online |
| Account Directory | Username resolution, public key distribution | Always online |
| STUN Servers | NAT traversal (connection establishment) | Free, public |
| TURN Servers | Media relay for restricted networks | Free tier + Premium |
| Update Server | Client and server software distribution | Always online |

### Auth Server

The centralized authentication server enables:

- Single account across all devices
- Session synchronization
- Device management (view/revoke sessions)
- Account recovery (with email backup)
- Public key package distribution for MLS

Users authenticate once with their VauxlNet account and can connect to any server (hosted or self-hosted) using the same identity.

**Important**: The Auth Server handles identity only. Message content, server membership, and conversation history are never transmitted to or stored on the Auth Server.

### STUN and TURN Infrastructure

Voice and video communication requires NAT traversal. The Vauxl team operates:

**STUN Servers (Free)**
- Globally distributed
- Unlimited use
- Enables peer-to-peer connections when possible

**TURN Servers (Tiered)**

| Tier | Bandwidth | Use Case |
|------|-----------|----------|
| Free | 5 GB/month per user | Casual use, small servers |
| Premium | 100 GB/month per user | Active communities |
| Enterprise | Unlimited | Commercial deployments |

Self-hosted servers can use Vauxl TURN infrastructure or deploy their own.

### Hosted Server Platform

The Vauxl team operates a multi-tenant platform where users can create small servers without deploying infrastructure.

**Free Tier Specifications**

| Resource | Limit |
|----------|-------|
| Servers per account | 3 |
| Members per server | 50 |
| Voice channel participants | 10 concurrent |
| File storage | 1 GB per server |
| Message history | 30 days |

**Premium Tier**

| Resource | Limit |
|----------|-------|
| Servers per account | Unlimited |
| Members per server | 500 |
| Voice channel participants | 50 concurrent |
| File storage | 50 GB per server |
| Message history | Unlimited |

Servers on the hosted platform use shared infrastructure but maintain full encryption. The Vauxl team cannot access message content.

## Deployment Models

Vauxl supports multiple deployment configurations to match different requirements.

### Model 1: Hosted (Vauxl Platform)

```
User Device ──▶ Vauxl Auth ──▶ Vauxl Platform Server
                    │
                    └──▶ Vauxl STUN/TURN
```

- No infrastructure required
- Account created on vauxl.net
- Servers created through web or app interface
- Limited by tier

**Best For**: Individuals, small communities, evaluation

### Model 2: Self-Hosted with Vauxl Auth

```
User Device ──▶ Vauxl Auth ──▶ Self-Hosted Server
                    │
                    └──▶ Vauxl STUN/TURN (or self-hosted)
```

- User deploys server on their own infrastructure
- Authentication via VauxlNet accounts
- Can use Vauxl STUN/TURN or self-host
- Full control over data and resources

**Best For**: Privacy-focused users, hobbyists, small organizations

### Model 3: Enterprise Managed

```
User Device ──▶ Vauxl Auth (or Enterprise IdP) ──▶ Vauxl-Managed Server
                                                   (in customer infrastructure)
```

- Server runs in customer data center or cloud
- Vauxl team manages software updates and maintenance
- Can integrate with enterprise identity providers (SAML, OIDC)
- Optional: Vauxl-operated or customer-operated STUN/TURN

**Best For**: Enterprises requiring data sovereignty with managed service

### Model 4: Enterprise Self-Managed

```
User Device ──▶ Enterprise Auth ──▶ Enterprise Server
                                   (fully isolated)
```

- Complete software package deployed by customer
- No connection to Vauxl infrastructure
- Customer manages updates, backups, operations
- Can operate behind VPN or in air-gapped network

**Best For**: Regulated industries, government, defense

### Model 5: LAN / Offline

```
User Device ──▶ Local Auth ──▶ LAN Server
              (within same network)
```

- Server discovered via mDNS/Bonjour on local network
- No internet required
- Temporary accounts or pre-provisioned users
- Ideal for events, temporary installations

**Best For**: LAN parties, conferences, field deployments

## Server Architecture

### The Server Manager Concept

A "Vauxl Server" is actually a Server Manager capable of hosting multiple virtual servers (instances). This architecture enables:

- Single deployment, multiple communities
- Resource sharing and isolation
- Simplified administration
- Rapid server creation

```
┌───────────────────────────────────────────────────────────────┐
│                      Server Manager                           │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │                    Admin Interface                      │  │
│  │  - Create/delete instances                              │  │
│  │  - Resource allocation                                  │  │
│  │  - Monitoring and logs                                  │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                               │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐      │
│  │  Instance A   │  │  Instance B   │  │  Instance N   │      │
│  │               │  │               │  │               │      │
│  │  - Channels   │  │  - Channels   │  │  - Channels   │      │
│  │  - Members    │  │  - Members    │  │  - Members    │      │
│  │  - Roles      │  │  - Roles      │  │  - Roles      │      │
│  │  - History    │  │  - History    │  │  - History    │      │
│  └───────────────┘  └───────────────┘  └───────────────┘      │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │                   Shared Services                       │  │
│  │  - Database (PostgreSQL)                                │  │
│  │  - Media Server (LiveKit)                               │  │
│  │  - File Storage                                         │  │
│  └─────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────┘
```

### Instance Isolation

Each instance operates independently:

- Separate encryption keys (MLS groups)
- Isolated message history
- Independent role and permission systems
- Distinct invite codes

Instances share underlying compute and storage but cannot access each other's data.

### First User Ownership

When a new instance is created or accessed for the first time:

1. First authenticated user becomes Owner
2. Owner has full administrative permissions
3. Owner can assign additional administrators
4. Similar to Teamspeak server model

This enables rapid setup without pre-configuration.

### Deployment Simplicity

**Docker Compose Deployment**

```yaml
# Example: docker-compose.yml
version: '3.8'
services:
  vauxl:
    image: vauxl/server-manager:latest
    ports:
      - "8443:8443"     # API/WebSocket
      - "10000:10000/udp"  # Media
    volumes:
      - vauxl_data:/data
    environment:
      - VAUXL_AUTH_URL=https://auth.vauxl.net
      - VAUXL_TURN_SERVERS=turn:turn.vauxl.net:3478

volumes:
  vauxl_data:
```

**One-Line Installer**

```bash
curl -fsSL https://get.vauxl.net | bash
```

The installer:
1. Detects operating system
2. Installs Docker if needed
3. Downloads and starts the server manager
4. Prints connection URL

## Licensing Model

Vauxl employs a "Quad-License Suite" to balance open-source freedom, community accessibility, and commercial sustainability.

See the LICENSE.md file in the repository root for complete legal terms.

### The Quad-License Suite

1.  **Core Software (AGPLv3)**: The Rust-based engine (`core/`). Open source, copyleft. Ensures the core technology remains free and verifiable.
2.  **App Software (Apache 2.0)**: The Flutter client (`app/`). Permissive. Allows developers to build custom clients, proprietary integrations, and branded desktops without restriction.
3.  **Server Software (Vauxl Fair Use)**: The backend services (`server/`). Sustainable. Free for personal and community use; paid for large commercial entities.
4.  **Documentation (CC BY-SA 4.0)**: The manuals and guides (`docs/`). Open content. Encourages community translation and improvement.

### Philosophy

1.  **Engine Freedom**: The core cryptographic and networking engine should be standard, verifiable, and open to all (AGPLv3).
2.  **Client Flexibility**: Developers should be able to build any experience they want on top of the generic core (Apache 2.0).
3.  **Sustainability**: The server infrastructure code allows Vauxl to capture value from large-scale commercial deployments to fund development (Fair Use).
4.  **Community First**: Documentation belongs to everyone (CC BY-SA 4.0).

### Server Software License Tiers (Vauxl Fair Use)

The **Server Software** is governed by the Vauxl Fair Use License, which includes the following tiers.

#### Personal License (Free)

**Who Qualifies**:
- Individuals using Vauxl for personal, family, or friend group communication
- No organizational affiliation required

**Rights**:
- Self-host on your own infrastructure
- Run unlimited server instances
- Scale to any capacity your hardware supports
- Modify the software for personal use
- Use Vauxl-operated Auth, STUN, and TURN services

**Obligations**:
- Non-commercial use only (no revenue generation)
- If you modify server software, publish your changes within 30 days

**Cost**: Free

---

#### Educational Institution License (Free)

**Who Qualifies**:
- Accredited educational institutions including primary schools, secondary schools, universities, and vocational training institutions
- Research institutions and academic libraries
- No employee count or revenue limits apply

**Rights**:
- Self-host on institution-owned or institution-operated infrastructure
- Provide servers for faculty, staff, researchers, and students
- Run unlimited server instances
- Use Vauxl-operated Auth, STUN, and TURN services
- Access optional support at cost (no markup)

**Obligations**:
- Use limited to educational and research purposes
- Not offered as a commercial service to external parties
- If you modify server software, publish your changes within 30 days

**Cost**: Free

**Note**: Educational institutions may exceed the 50 employee threshold and still qualify for this license. The key requirement is that the institution is an accredited educational or research organization using Vauxl for its educational mission.

---

#### Startup License (Free)

**Who Qualifies**:
- Organizations with **fewer than 50 employees** AND **less than EUR 1,000,000 annual revenue**
- Includes startups, small businesses, and nonprofits

**Rights**:
- Self-host for internal organizational use
- Run unlimited server instances
- Use Vauxl-operated Auth, STUN, and TURN services
- Access optional support at cost (no markup)

**Obligations**:
- Internal use only (not offered as a service to external parties)
- Notify Vauxl when approaching thresholds

**Internal Use Exemption**: Modifications made solely for internal use (not deployed to third-party-accessible infrastructure) are exempt from source disclosure requirements. This allows startups to customize without mandatory publication.

**Cost**: Free

**When Thresholds Are Exceeded**: Contact Vauxl for Commercial or Enterprise licensing.

---

#### Community Hoster License (Free)

**Who Qualifies**:
- Individuals or groups operating public Vauxl servers for community benefit
- Revenue must not exceed infrastructure costs plus EUR 25,000 annual margin

**Rights**:
- Run public servers accessible to anyone
- Charge users to cover operational costs
- Retain up to EUR 25,000 above direct infrastructure costs annually

**Obligations**:
- Register annually with Vauxl as a Community Hoster
- Adhere to Vauxl Fair Pricing Guidelines (published reference rates)
- Display Vauxl attribution in the service
- If you deploy server modifications, publish your changes within 30 days
- Operate transparently (costs and pricing visible to users upon request)
- Provide documentation upon request for margin verification

**Cost**: Free

**Definition of Infrastructure Costs**: Server hosting, bandwidth, storage, domain registration, and external cloud services. **Excludes**: personnel costs, salaries, contractor fees, management fees, and corporate overhead.

**Fair Pricing Guidelines**: Vauxl publishes maximum per-user pricing adjusted annually based on reference infrastructure costs. Community Hosters must not exceed these rates.

---

#### Commercial License (Required)

**Who Must Obtain Commercial License**:
- Organizations with **50 or more employees** OR **EUR 1,000,000 or more annual revenue**
- Any entity operating public hosting with margin exceeding EUR 25,000 annually
- Any entity offering Vauxl-based services to third parties for profit

**How Commercial Use Works**:
- Commercial users obtain Vauxl services through the Vauxl Platform (SaaS)
- Self-hosting is available through Enterprise License only

**Pricing**: Subscription based on usage (users, servers, features)

**Includes**:
- Vauxl-managed hosting on shared or dedicated infrastructure
- Commercial support with SLA
- Access to premium TURN bandwidth

---

#### Enterprise License

**Who This Is For**:
- Large organizations requiring on-premise deployment
- Organizations with data sovereignty, compliance, or regulatory requirements
- Any commercial entity requiring self-hosted infrastructure

**Deployment Options**:
- Vauxl-managed in customer cloud (AWS, Azure, GCP)
- Vauxl-managed in customer data center
- Customer self-managed with Vauxl support
- Air-gapped / fully isolated

**Includes**:
- Full software license for self-hosting
- Deployment assistance
- SLA with guaranteed response times
- Priority support with dedicated contact
- Custom integrations (consulting available)
- Training and onboarding

**Pricing**: Custom, based on scale and requirements. Contact sales@vauxl.net.

---

### Source Disclosure Requirement

**Core Software (AGPLv3)**:
> If you modify the Core Software and use it to provide a service over a network, you must explicitly offer the source code to all users of that network (standard AGPLv3 requirement).

**Server Software (Fair Use)**:
> If you deploy modified Server Software to infrastructure accessible by third parties, you must publish the source code of the modifications within 30 days.

**Internal Use Exemption**: Modifications used solely for internal operations (not accessible to third parties) are exempt.

**Scope**: Client software (Apache 2.0) is exempt from disclosure requirements.

---

### Support Tiers

| Tier | Availability | Price Model |
|------|--------------|-------------|
| Community | Forums, documentation, GitHub issues | Free |
| Startup Support | Direct engineering assistance | At cost (EUR 50/hour, no markup) |
| Commercial Support | Business hours, email and chat | Included with Commercial License |
| Enterprise Support | 24/7, phone, dedicated engineer, SLA | Included with Enterprise License |

---

### License Determination Guide

| Your Situation | License | Self-Host Allowed | Cost |
|----------------|---------|-------------------|------|
| Individual, personal use | Personal | Yes | Free |
| Educational institution (school, university, research) | Educational Institution | Yes | Free |
| Startup, under 50 employees AND under EUR 1M revenue | Startup | Yes | Free |
| Nonprofit under thresholds | Startup | Yes | Free |
| Running public servers at-cost for community | Community Hoster | Yes | Free |
| Company with 50+ employees (not educational) | Commercial/Enterprise | Via Vauxl or Enterprise | Paid |
| Company with EUR 1M+ revenue | Commercial/Enterprise | Via Vauxl or Enterprise | Paid |
| Making profit (>EUR 25k margin) from hosting | Commercial | Via Vauxl | Paid |
| Requiring on-premise for compliance | Enterprise | Yes | Custom |

---

### Enforcement and Good Faith

Vauxl operates on a good-faith model:

1. **Self-Declaration**: Users self-declare their license tier
2. **Audit Rights**: Vauxl reserves the right to verify compliance for Commercial/Enterprise tiers
3. **Grace Period**: Organizations exceeding thresholds have 90 days to contact Vauxl
4. **Disputes**: Resolved through direct communication before legal action

---

### Data Protection Requirements

All operators of Vauxl servers must comply with applicable data protection laws.

**Privacy by Design**:
- All message content is end-to-end encrypted (MLS protocol)
- Servers are "dumb storage" and never have access to plaintext content
- All data at rest is encrypted
- Metadata collection is minimized to prevent profiling

**Compliance**:
- EU deployments must comply with GDPR
- German deployments must comply with BDSG (Bundesdatenschutzgesetz)
- Operators must respond to data subject requests as required by law

See the LICENSE file (Section 8: Data Protection and Privacy) for complete requirements.

---

### Security Commitment

**No Backdoors**:

The Vauxl Team will not implement any backdoor, master key, key escrow, or other mechanism that would allow unauthorized access to encrypted user communications. This applies regardless of governmental requests, court orders, or legislation.

If any jurisdiction mandates backdoor access:
- Vauxl will not comply
- Vauxl will pursue legal challenges
- If unavoidable, Vauxl will cease services in that jurisdiction rather than compromise security

**Transparency**:
- Vauxl publishes transparency reports on government data requests
- The open-source codebase enables independent security audits
- Users can verify the absence of backdoors

See the LICENSE file (Section 9: Security Principles) for the complete commitment.

## Enterprise Offerings

### Vauxl Enterprise SaaS

For organizations that require managed infrastructure without operational burden.

**Included**:
- Dedicated server manager instance
- Guaranteed resource allocation
- Integration with enterprise identity (SAML 2.0, OIDC)
- Audit logging and compliance reports
- 99.9% uptime SLA
- Priority support (4-hour response)

**Data Residency Options**:
- EU (Frankfurt)
- US (Virginia)
- Asia-Pacific (Singapore)
- Custom (Enterprise tier)

### Vauxl Enterprise On-Premise

For organizations requiring complete data sovereignty.

**Deployment Options**:

| Option | Description |
|--------|-------------|
| Customer Cloud | Runs in customer AWS/Azure/GCP subscription |
| Customer Data Center | Runs on customer-owned hardware |
| Air-Gapped | No internet connectivity |

**Support Packages**:

| Package | Includes |
|---------|----------|
| Standard | Email support, business hours, 8-hour response |
| Premium | Phone and email, 24/7, 2-hour response |
| Dedicated | Assigned engineer, proactive monitoring |

### Enterprise Identity Integration

Enterprise deployments can integrate with existing identity systems:

- SAML 2.0
- OpenID Connect
- LDAP/Active Directory (via adapter)
- Custom SSO (consulting engagement)

Users authenticate with corporate credentials. No separate VauxlNet account required for on-premise deployments.

## Offline and LAN Deployment

### Use Cases

- LAN parties with no internet access
- Conference installations
- Military or secure facility communication
- Remote site operations
- Disaster recovery scenarios

### Architecture

In offline mode, the server manager includes:

- Local authentication server
- Local STUN server (mDNS discovery)
- Optional local TURN server (for complex network topologies)

```
┌─────────────────────────────────────────────────────────────────┐
│                         LAN Network                             │
│                                                                 │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐                 │
│   │ Client A │    │ Client B │    │ Client N │                 │
│   └────┬─────┘    └────┬─────┘    └────┬─────┘                 │
│        │               │               │                        │
│        └───────────────┼───────────────┘                        │
│                        │                                        │
│                        ▼                                        │
│            ┌───────────────────────┐                            │
│            │   LAN Server Manager  │                            │
│            │                       │                            │
│            │   - Local Auth        │                            │
│            │   - STUN (mDNS)       │                            │
│            │   - Server Instances  │                            │
│            │   - Media Server      │                            │
│            └───────────────────────┘                            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### LAN Discovery

Clients discover servers on the local network via:

1. mDNS/Bonjour announcement
2. Manual IP entry
3. QR code scan (displayed on server console)

### Temporary Accounts

For events without pre-registration:

- Users create temporary accounts on the LAN server
- Accounts exist only for the event duration
- No VauxlNet account required
- Optional: Export account to VauxlNet after event

### Synchronization After Reconnection

When a LAN server gains internet connectivity:

- Message history can be optionally backed up
- Users can link temporary accounts to VauxlNet accounts
- Server can federate with other servers (future feature)

## Account and Identity System

### VauxlNet Accounts

The VauxlNet identity system provides:

- Single identity across all Vauxl servers
- Cross-device synchronization
- Cryptographic key management
- Username reservation

### Account Creation

1. User registers at vauxl.net or via app
2. Client generates identity keypair locally
3. Public key uploaded to Auth Server
4. Username linked to public key

**Privacy Note**: The Auth Server stores only public keys and minimal metadata. Private keys never leave the user's devices.

### Device Linking

To use an account on a new device:

1. Authenticate on new device
2. Verify from existing device (cross-signing)
3. MLS key packages generated for new device
4. New device can join existing groups

### Account Portability

For users preferring self-custody:

- Export identity keys (encrypted backup)
- Import on new device without Auth Server
- Reduced convenience, maximum privacy

### Federated Identity (Future)

Planned support for:

- Self-hosted Auth Server federation
- WebFinger discovery
- Cross-instance identity verification

## Pricing Summary

Vauxl aims for sustainability, not profit maximization. Pricing covers infrastructure and development costs.

### Vauxl Hosted Platform

| Tier | Price | Includes |
|------|-------|----------|
| Free | EUR 0 | 3 servers, 50 members each, 5GB TURN/month |
| Premium | EUR 5/month | Unlimited servers, 500 members, 100GB TURN/month |
| Dynamic | Usage-based | Pay for actual resource consumption |

### Self-Hosted (Personal, Startup, Community Hoster)

| Tier | Price | Includes |
|------|-------|----------|
| Software | EUR 0 | Full software, use Vauxl Auth/STUN/TURN |
| Community Support | EUR 0 | Forums, documentation, GitHub issues |
| Direct Support | EUR 50/hour | Engineering assistance at cost |

### Commercial

| Tier | Price | Includes |
|------|-------|----------|
| Commercial SaaS | Subscription (usage-based) | Managed hosting, commercial support, SLA |

### Enterprise

| Tier | Price | Includes |
|------|-------|----------|
| Managed SaaS | Custom | Dedicated infrastructure, premium SLA |
| On-Premise | Custom | Full license, deployment, support contract |
| Air-Gapped | Custom | Offline package, optional on-site support |

Contact sales@vauxl.net for Commercial and Enterprise pricing.

## Appendix: Deployment Checklist

### Self-Hosted Minimum Requirements

| Resource | Specification |
|----------|---------------|
| CPU | 2 cores |
| RAM | 4 GB |
| Storage | 20 GB SSD |
| Network | 100 Mbps, static IP or DDNS |
| OS | Linux (Debian 11+, Ubuntu 22.04+) |

### Ports Required

| Port | Protocol | Purpose |
|------|----------|---------|
| 443 | TCP | HTTPS/WSS (API and signaling) |
| 8443 | TCP | Alternative HTTPS (optional) |
| 3478 | UDP/TCP | STUN/TURN |
| 10000-10100 | UDP | Media (configurable range) |

### DNS Configuration

For public deployments:
- A record pointing to server IP
- Wildcard or subdomain for instances (optional)
- Valid TLS certificate (auto-provisioned via Let's Encrypt)
