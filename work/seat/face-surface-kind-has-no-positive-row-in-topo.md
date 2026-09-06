---
id: face-surface-kind-has-no-positive-row-in-topo
kind: issue
title: query::face_surface_kind and face_surface_matches have no positive row in topo — every Some/true assertion lives downstream in sweep's tests
status: open
opened: 2026-09-06
---


## What

Found by TOPO's two-homes unit (PR 1959) when it mutated
`readback::face_carrier_kind` to refuse every face: in `topo` only one
row went red (`query::tests::the_adjacent_pair_is_unordered`, which
reaches the door through `face_kind_across`). `face_surface_kind`'s own
rows are negative only — `None` on a dangling key
(`crates/topo/src/query.rs:1181-1182`) — and `face_surface_matches`'
are `false` for an empty set (`:1187`, `:1283`). Every "Some on a live
face" / "true for a matching kind" assertion lives downstream:
`crates/sweep/tests/verbs_cylsph_opening.rs:169`,
`crates/sweep/tests/verbs_sphsph_chart.rs:124`,
`crates/sweep/tests/mate7a_r1_probes.rs:115`. A positive row belongs
in this program's file. Filed by the TOPO orchestrator, 2026-09-06.
