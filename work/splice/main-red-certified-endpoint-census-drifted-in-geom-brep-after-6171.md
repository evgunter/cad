---
id: main-red-certified-endpoint-census-drifted-in-geom-brep-after-6171
kind: issue
title: main red (slow set): geom-core's certified_endpoint_census drifted in four geom-brep files after PR 6171 (spline_face, krawczyk, pair_section, ssi/enclose)
status: open
opened: 2026-11-01
priority: P1
cost: E
---


Filed by the CLEAR orchestrator. CLEAR's TRV lane found it while
re-pinning `injective.rs` and `clearance.rs` in the census (PR 6233).
The census is a slow-set test, so PR gates do not run it.

## What

`crates/geom-core/tests/certified_endpoint_census.rs` (`ROSTER`) is red
on main. Four geom-brep files no longer match their pins (counts as
found by the TRV lane, against the pin):

| file | found | pinned |
|---|---|---|
| `geom-brep/src/curve_face/spline_face.rs` | (23, 23) | (9, 8) |
| `geom-brep/src/krawczyk.rs` | (4, 3) | (5, 4) |
| `geom-brep/src/.../pair_section.rs` | present | not pinned |
| `geom-brep/src/ssi/enclose.rs` | (21, 21) | (19, 19) |

The drift arrived after SPLICE's PR 6171. The earlier census row,
`main-red-certified-endpoint-census-moved-in-six-files`, is closed.

## What closing it takes

The same as the closed row: re-run the census on main, attribute each
moved count to its PR, disposition the new reads, and re-pin.
