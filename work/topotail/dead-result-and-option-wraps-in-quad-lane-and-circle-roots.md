---
id: dead-result-and-option-wraps-in-quad-lane-and-circle-roots
kind: issue
title: props::quad_lane's chan and translated_curve, and boolean::circle_roots' certified_subdivision, wrap an answer no exit leaves unfilled
status: open
opened: 2026-10-04
priority: P4
cost: E
---

Found by the review-fix pass on PR 4033 (`topo/remaining-torn-body-refusals-unreachable`),
whose sweep for dead `Result`s ran
`cargo clippy -p topo --lib --all-features -- -A clippy::all -W clippy::unnecessary_wraps`.
That PR fixed the five hits its own diff made (`boolean/ops.rs`'s
`event_pairs`, `pair_verdict`, `walk_pairs`, `sphere_faces_apart`, and
`boolean/reduce.rs`'s `edges_share_a_curve`). These three predate it:

- `crates/topo/src/props/quad_lane.rs`, `chan`: returns a `Result` every
  exit fills with `Ok`.
- `crates/topo/src/props/quad_lane.rs`, `translated_curve`: returns an
  `Option` every exit fills with `Some`.
- `crates/topo/src/boolean/circle_roots.rs`, `certified_subdivision`
  (CLEAVE/HONE ground): returns a `Result` every exit fills with `Ok`.

A wrap no exit uses makes each caller handle a refusal that cannot
arrive, and it reads as a fallible step. Either drop the wrap, or say at
the site which future refusal it is held for.

## Re-homed at TOPO's cut (2026-10-11)

Moved from TOPO to TOPOTAIL when TOPO dropped to P2 and split on its priority seam (`work/topotail/log.md`). TOPO kept the Euler surgery doors; this row is a torn-record, refusal-recourse or code-hygiene row, or a parked boolean or placement row.
