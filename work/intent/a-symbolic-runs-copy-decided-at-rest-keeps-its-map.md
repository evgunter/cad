---
id: a-symbolic-runs-copy-decided-at-rest-keeps-its-map
kind: issue
title: A symbolic run's copy decided at rest keeps its map, so a drive's leaves re-certify what the witness build snapped
status: closed
opened: 2026-10-18
priority: P2
cost: M
closed: 2026-10-20
---

Found by PR 5267 (s3 C), measuring the orchestrator's antipode question on the tour's plate study.

**Measured.** The study ran with the plate's copy placed by the gesture (`place_at` at the identity) instead of the named interim's world pin. The anchor rule took the start cap (`Anchor::Fixed`, `PlacedBy::PlanarFace`), so the map is the identity at every point of the box. Of the drive's 512 leaves, 301 were refused `budget` and 211 `flip_crossing`. Every one of the 211 diverged at the placement node by count, not by sign: `transform_rigid_*` (7 predicates), `carrier_endpoint_{start,end}`, `carrier_on_surface_{1,2}`, `dihedral_{arm,wedge}`, `interval_span_forward` and `witness_*` were decided 0 times at the witness and 12 to 108 times in the leaf. `mate_chart_reference_antipode` is not among them. With the world pin, the same study has 301 `budget` leaves and no `flip_crossing`.

**Mechanism.** The fold's snap (`crates/editor-core/src/eval/wire.rs`, the `rest_verdict` match) places a copy decided at rest by no map where the run carries no variation, and otherwise by `SolveScalar::identity_varying_as`. That is `None` for `Sym` (`crates/editor-core/src/eval/mod.rs`), so the map is kept. A drive's symbolic leaves therefore apply and re-certify a map that the witness build dropped, and the verdict vectors differ.

**Open.** A `Sym` zero decided as a theorem says the displacement is zero over the DAG, so the map could be dropped with nothing lost; a zero decided only numerically could not. Whether `identity_varying_as` can tell the two apart, or the drive should compare a placement's decisions modulo the snap, is the question. The interim world pin covers the tour and the corpus; a copy the gesture places at a pose off the identity is not covered.

## Closed — no lane decides at rest any more (D, FORK-REST)

FORK-REST removed the lane-dependent snap this item describes. Whether a copy is placed by no map is read off its recipe (the identity bundle as written), never decided from values, so the `f64` witness, a box lane and a `Sym` leaf place every copy the same way: the identity bundle by no map, every other placement by its map, even at the exact identity. `SolveScalar::identity_varying_as` and the other rest hooks are deleted, so `Sym` has no separate answer to give.

The study this item measured is the evidence: the tour's plate study is 211/512 certified, with no `flip_crossing` refusal (its 301 others are `budget`), and the 1-link chain drive, which refused `flip_crossing` under the old snap, certifies its whole box. The question "a `Sym` zero decided as a theorem versus numerically" no longer arises, since no zero is decided.
