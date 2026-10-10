---
id: a-far-origin-rod-cut-escalates-its-mapped-source-at-eps-1e-12
kind: issue
title: At ε 1e-12 the far-origin rod's cut by a brick escalates carrier_matches_mapped_source (red on main)
status: open
opened: 2026-10-10
priority: P2
cost: M
---


`crates/sweep/tests/parallel_cylinder_join.rs`'s
`a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings` fails
under `CAD_TOLERANCE_EPS=1e-12`. It fails on main (origin/main at
`4868268ba4`, measured 2026-10-10) and on `intent/s4-e-glue-on-zero`
alike, and passes at the default ε and at 1e-6.

The failure is in the fixture itself, not in the scene under test.
`cut_and_tipped` (`parallel_cylinder_join.rs:383`) intersects a rod
whose wall origin is stored `far` along its axis with a brick, and that
intersect refuses:

```
CrossingInsertion { operand: A, edge: EdgeKey(3v1), source: Certification { error: Escalated {
  check: MappedSource, sample: 3, cause: Indeterminate { margin: 1.2533307724993392e-12,
  band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("carrier_matches_mapped_source") } } } }
```

The certifier's mapped-source residual (`geom-brep` `certify.rs` /
`mapped.rs`) lands just past the zero band at 1e-12. That is the far
stored origin's lever on a residual that scales with the wall's
parameter range. CI's ε-extra job runs only for the crates a diff
seeds, so the row has not run at 1e-12 since PR 4231 added it.

Either the residual's floor has to be read against the lever `far`
puts on it, or the fixture keeps `far` within what the band
certifies at 1e-12, with that limit stated beside it.
