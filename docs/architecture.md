# WireSentinel-ZTNA Architecture

Phase 15 Zero Trust Network Access layer — identity, device trust, conditional access, and private application publishing.

## Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     WireSentinel Agent                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────────────┐ │
│  │ identity │  │  trust   │  │  policy  │  │    gateway      │ │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────────┬────────┘ │
│       │             │             │                  │          │
│  ┌────┴─────────────┴─────────────┴──────────────────┴──────┐  │
│  │ publishing · segmentation · connectors · analytics · sdk  │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                    ZtnaHeartbeatPayload
                    ZtnaPolicyBundleDto
                              │
                              ▼
                   WireSentinel-Controller
```

## Core abstractions

### `IdentityManager`

Registers multiple `IdentityProvider` implementations (Local, OIDC, OAuth2, SAML/LDAP mocks, Azure AD, Google Workspace, Okta, Keycloak). Generic OIDC providers fetch discovery documents via `reqwest`.

### `DeviceTrustEngine` / `TrustScoreEngine`

Maintains `DeviceTrustRecord` entries and computes 0–100 trust scores from `DevicePosture` signals.

### `ConditionalAccessEngine` / `ZtnaPolicyEngine`

Evaluates `ZtnaSecurityPolicy` rules with `Condition`/`Action` semantics and returns `ConditionalAccessResult`.

### `ServiceGateway`

HTTP reverse-proxy stub (axum) and TCP relay stub. Emits `GatewayConnectionEstablished` / `GatewayConnectionDenied` via `shared-types` events.

### `ResourcePublisher`

In-memory catalog with optional SQLite persistence (`sqlite` feature + sqlx).

### `MicroSegmentationEngine`

Defines `MicroSegment` boundaries and emits `SegmentPolicyApplied` / `SegmentPolicyDenied`.

### `ApplicationConnector`

Outbound connector registration with `ConnectorHealth` monitoring.

### `ZtnaAnalytics`

Records access decisions and produces `ZtnaAnalyticsSnapshot` rollups.

## Controller integration

Agents report `ZtnaHeartbeatPayload` and receive `ZtnaPolicyBundleDto` from WireSentinel-Controller.

## Crate dependency graph

```
ztna-core ──┬── identity
            ├── trust
            ├── policy
            ├── gateway
            ├── connectors
            ├── segmentation
            ├── publishing
            ├── analytics
            └── sdk

controller (DTOs only)
```

## External dependencies

| Repository | Used by |
|------------|---------|
| `WireSentinel/shared-types` | DTOs (`phase15`), service events |

## Phases covered

| Phase | Crate(s) |
|-------|----------|
| 15-A | `ztna-core` |
| 15-B | `identity` |
| 15-C | `trust` |
| 15-D | `policy` |
| 15-E | `gateway` |
| 15-F | `publishing` |
| 15-G | `segmentation` |
| 15-H | `connectors` |
| 15-I | `analytics` |
| 15-J | `sdk` |
| 15-K | `controller` |
