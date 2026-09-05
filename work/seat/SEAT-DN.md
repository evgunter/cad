---
id: SEAT-DN
kind: unit
title: direction normalization: one kernel body under two ratified funnel names (Ev's ruling B)
status: closed
opened: 2026-09-05
branch: seat/dirnorm
refs: [direction-normalization-two-doors-one-home, 1902, 1564]
pr: 1987
closed: 2026-09-05
---


Spec: `docs/SEAT-DN-SPEC.md` (deleted at merge per `docs/DOC-LEDGER.md`).
Executes Ev's ruling (B) on `[ev]` PR 1902: ONE decide/normalize/refuse
body in `topo::query` taking the funnel-site name as a parameter;
`UnitVec3::new` and `editor-core`'s `unit()` become calls to it under
their existing names, so no K-REPORT row or pinned census count moves;
the mate re-read's two-name split restated as ratified; `Dir::from_unit`
given issue-1527's treatment after measuring its callers. Closes
`direction-normalization-two-doors-one-home`. Block SEAT-B3 slot 2.

## Closed

PR 1987 (2026-09-05). Delivered as specified; the dual found zero
code defects in the unit and one class its completeness claim missed
(live decide-then-normalize siblings in `geom-core`, `sweep` and
`topo::sector_shape` that ask no finiteness question — re-scoped into
`two-d-director-doors-skip-the-finiteness-question`, FIX's
`is-finite-length-homed-in-the-query-seat` the ruling that closes it).
Record: `work/seat/log.md` "SEAT-DN MERGED"; MODEL-AB-LOG row SEATDN
(ordinal 1010, sample #144).
