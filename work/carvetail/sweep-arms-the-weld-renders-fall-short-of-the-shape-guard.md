---
id: sweep-arms-the-weld-renders-fall-short-of-the-shape-guard
kind: issue
title: The extrude and revolve SmoothJoinRefuted arms name their edge by key, and RevolveError::PinnedRunStation states no recourse
status: open
opened: 2026-10-20
priority: P4
cost: E
---


Found by `wire-census2`, whose PR welds the feature-tree census's
`Extrude/` and `Revolve/` rows to `sweep::ExtrudeError`'s and
`sweep::RevolveError`'s arms (`every_forwarded_arm_has_a_row`, `crates/editor-core/tests/refusal_concision_chains.rs:1389`). The arms the
roster turned up with no row render short of the standard:

- **`SmoothJoinRefuted`**, both doors (`crates/sweep/src/extrude.rs:462`,
  `Display` at `:570`; `crates/sweep/src/revolve/mod.rs:619`, `Display`
  at `:904`), names its edge by key: "along
  the join EdgeKey(null), the faces meet at a corner at sample 3 and
  smoothly at sample 4, …". A smooth-join verdict is a fact about the
  geometry, not a corrupt body, so the key is not what the person
  needs. On `ADMISSIONS` under a comment citing this file.
- **`RevolveError::PinnedRunStation`** (`revolve/mod.rs:632`,
  `Display` at `:919`) states no recourse: "loop 0
  vertex 2 joins two walls of one run but lies on the axis, so the
  run's wall has no strut there". On `FILED_NO_RECOURSE` under a
  comment citing this file.

`ExtrudeError::Pcurve` names its face by key, as `Revolve/Pcurve` and
`Loft/Pcurve` do; it joins them on `KERNEL_KEYED` (`crates/editor-core/tests/refusal_concision_chains.rs:45`).

## Repair shape

Name the edge in words (or drop it, as the split's own
`SmoothJoinRefuted` row does) and end `PinnedRunStation` in its
recourse, then drop the admissions.
