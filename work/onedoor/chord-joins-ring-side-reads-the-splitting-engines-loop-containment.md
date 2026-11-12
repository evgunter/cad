---
id: chord-joins-ring-side-reads-the-splitting-engines-loop-containment
kind: issue
title: the in-plane loop containment (splitting::containment, its reader, answer, error and payloads) is read from outside the splitting engine by chord_join, census, validate and boolean
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found by `onecopy/boolean-standing` (PR 6576), whose ring-side pin raised
`chord_join.rs`'s and `chord_join/ring_side_rows.rs`'s `names-splitting`
pins (`scripts/gates/engine-leaves.counts`, which cites this row).

## What

`splitting::containment` is the in-plane loop containment: the reader
`point_in_loop`, its answer `LoopContainment`, its error
`PointInLoopError` with its standing (`PointInLoopError::refusal`), and
the payload types that error carries (`LoopDecision`, `Escalation`,
`Uncrossable`, `UncrossableCarrier`). Every one is read from outside the
engine, and `lib.rs` re-exports them:

- `chord_join`: `ring_side`'s `vertex_side` reads the reader, answer and
  error, and `chord_join/ring_side_rows.rs` pins it on the error's arms.
  Those two files are the cited rises in
  `scripts/gates/engine-leaves.counts` that this row retires.
  `chord_join/sibling_escalation_rows.rs` reads them too.
- `census`, `validate` and `test_support_samples`: the error and every
  payload.
- `boolean`: `contain`, `solid_contain`, `region`, `torus_region`,
  `spline_region`, `boxes`, `refusal_routes`, `reduce`, `ops`, `mod` and
  `carrier_touch` read the error, `Escalation` or `Uncrossable`.

`[topo.homes]` (`crates/topo/README.md`) says a reading the engine
hosts, which takes a body, belongs at the crate root.

## What closing it takes

Move the whole `splitting::containment` module (about 3,300 lines: the
reader, its answer, its error with its standing, and the payload types)
to the crate root. Then re-point `splitting` and every reader above at
it, and drop the cited rises and every other `names-splitting` pin the
move clears. The cited rises include `contain_loop.rs`, the containment
door's `From<PointInLoopError> for ContainError` (moved out of
`boolean/contain.rs` by PR 6683). It is mechanical, but it touches about twenty files, so it
costs M.
