---
id: a-ladder-or-annulus-rims-mate-with-a-ring-refuses-at-its-resolver
kind: issue
title: A ladder or annulus rim whose mate carries a ring refuses at its resolver
status: open
opened: 2026-11-12
priority: P3
cost: M
refs: [a-mixed-rims-mate-with-a-ring-refuses-at-the-mate-gate]
---

Filed by the band-mixgates lane (2026-11-12), sweeping for siblings of the mixed rim's mate-ring gate. That gate was lifted in the PR that closed `a-mixed-rims-mate-with-a-ring-refuses-at-the-mate-gate`.

Three closed-rim resolvers in `crates/sweep/src/blend/surgery.rs` still refuse `UnsupportedChain` whenever a mate face has a ring, however far the ring lies from the strip:
- `resolve_rim`'s ladder gate, "a rim's curved support carries rings of its own";
- `resolve_seam_split_rim`, "a seam-split rim's support carries rings of its own";
- `resolve_annulus`, "a rim's second support carries rings of its own".

`open/ruled.rs`'s gate on a non-planar support's rings is the open band's own, and is a fourth of the same shape.

The mixed carve showed the mechanism: a trim `mef` hands the strip to the new face, and the old face keeps its key and its rings. `support_boundary_clearance` meters every ring of every support. Each of these walks needs the same measurement before its gate goes: a witness with a ring on the curved mate (a pin hole through a boss's wall below its rim), the topology, tier 3, and ΔV against the sequential recourse.
