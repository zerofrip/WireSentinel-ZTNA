# WireSentinel-ZTNA

Phase 15 Zero Trust Network Access layer for WireSentinel.

## Crates

| Crate | Purpose |
|-------|---------|
| `ztna-core` | Core types, security policy, errors |
| `identity` | Identity providers and authentication |
| `trust` | Device trust and posture scoring |
| `policy` | Conditional access and ZTNA policy engines |
| `gateway` | HTTP/TCP service gateway stubs |
| `connectors` | Outbound application connectors |
| `segmentation` | Micro-segmentation engine |
| `publishing` | Resource catalog and access policies |
| `analytics` | Access decision analytics |
| `sdk` | ZTNA plugin trait and manifest |
| `controller` | Controller heartbeat and policy bundle DTOs |

## Build

```bash
cargo test --workspace
```

Requires `WireSentinel/shared-types` as a sibling repository.

## License

Apache-2.0. See [LICENSE](LICENSE).
