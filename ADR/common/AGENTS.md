common/AGENTS.md

Architecture
- Follow ADRs
- Do not introduce heap allocation - use heapless for static allocation
- Use `embassy_sync::watch` for shared state because receivers need the latest value rather than every historical update. Use channels only when every message must be delivered exactly once to a receiver.
- Prefer static allocation as this is an embedded system.

Coding style
- Small modules.
- No unsafe unless already present.
- Keep tasks under 200 LOC.
