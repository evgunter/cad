---
id: tests-common-body-fixtures-triplicated
kind: issue
title: Three crates' tests/common each carry the same six body fixtures; three pairs are byte-identical and three have drifted
status: open
opened: 2026-09-15
---

## Finding

- **Where**: `crates/mesh/tests/common/mod.rs`, `crates/stl/tests/common/mod.rs`,
  `crates/step-export/tests/common/mod.rs`
- **Importance**: medium
- **Confidence**: sure — the identical/drifted split below was measured, not grepped
- **Raised by**: the `S52` lane (SUITE), 2026-09-15, on a census of the
  `tests/common/` trees supplied by a sibling review lane

Six body fixtures are declared in more than one of the three trees. **The
split matters more than the count**, because it is the drift the duplication
class predicts, caught midway:

| fixture | mesh | stl | step-export | |
| --- | --- | --- | --- | --- |
| `ball` | ✓ | ✓ | ✓ | mesh and stl byte-identical; step-export differs |
| `donut` | ✓ | ✓ | ✓ | mesh and stl byte-identical; step-export differs |
| `l_prism` | ✓ | ✓ | — | byte-identical |
| `cone` | ✓ | ✓ | ✓ | **all three differ** |
| `washer` | ✓ | ✓ | ✓ | **all three differ** |
| `holed_prism` | ✓ | ✓ | — | **differ** |

So three of the six have already drifted, and nobody chose the drift: each
tree's copy was edited for its own suite. Whoever takes this reconciles rather
than merges — for each of `cone`, `washer` and `holed_prism`, decide which
body is right and say what moved, because at least two suites are metering a
shape they did not mean to be metering.

**The home exists and is reachable from all three crates.**
`crates/sweep/src/test_support.rs` is generic over the scalar and already
carries `cube`, `brick`, `prism`, `prism_at`, `prism_on`, `extruded`, `dome`,
`waisted`, `ball_poled_z`, `lantern`, `spool` and `sphere_zone`; `mesh`,
`step-export` and `stl` each already carry
`sweep = { path = "../sweep", features = ["test-support"] }` in
`[dev-dependencies]`, so joining costs a `use` and no manifest change. `ball`,
`donut`, `cone` and `washer` are revolves and sit beside `dome`; `l_prism`
and `holed_prism` are one and two loops through `extruded`.

The same census names `axis_y` ×2, `validated` ×2 and `p2` ×3 across the same
trees. Those are three-line vocabulary rather than fixtures and the routing
rule (`crates/sweep/src/test_support.rs`'s header) argues they stay local —
worth deciding explicitly rather than sweeping in.

Related, on this slate: `work/tint/sweep-boolean-suite-brick-and-prism-copies.md`,
whose `brick` half `S52` closed and whose `prism(pts, h)` half is still
open against the same home.
