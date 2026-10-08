---
id: run-loops-of-split-and-pierce-are-twins
kind: issue
title: splitting/insert.rs's run loop and boolean/vtxfac.rs's are near-verbatim twins: one home?
status: open
opened: 2026-10-06
priority: P3
cost: M
---



Found by PR 4096's review (2026-10-06).

Two places implement the same run scan:

- `crates/topo/src/splitting/insert.rs`: `above_runs` and
  `insert_null_edges`.
- `crates/topo/src/boolean/vtxfac.rs`: `out_runs` and the
  piercing-side runs loop in `classify_vertex_on_face`.

What they share:

- The enumeration: maximal cyclic runs over one class, anchored at the
  first entry of the opposite class.
- The real-member filter.
- `run_site`'s `Fan` and `WholeOrbit` arms.
- The lone-bisector strut at `after`, now with the same invariant and
  the same `unreachable!` check in both.

What differs is the entry type (`SectorEntry` with `SectorEntryKind`,
against vtxfac's `Entry` with `is_edge`), the class
(`PlaneSide::Above` against `SideCode::Out`), the side and attribute
derivation (the split always takes `Above` except at a whole orbit;
vtxfac reads the facing through `strut_faces_first`), and the record
type. Both lanes allow any number of runs; vtxfac also hangs its ring
struts in the runs' angular order (`vtxfac::ring_order`).

Owed: decide whether the enumeration and the site choice (fan, whole
orbit, or strut at `after`, with its check) belong in one helper over
`(he, is_real, in_run)`, which both lanes would call. Each lane would
keep its own side and record. If they share a home, the proof at the
strut site is stated once. If not, say why the two must stay separate.
