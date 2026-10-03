---
id: a-mate-frame-axis-is-decided-against-a-length-band
kind: issue
title: A mate side's composed axis is re-minted against the run's length band, so at a coarse epsilon every non-identity frame offset refuses whatever the part's scale
status: parked
opened: 2026-10-03
priority: P3
cost: M
design: true
blocked_on: [3990]
---



Found in review of PLACE's mate-frame-offset unit (PR 3961), by both
reviewers; disclosed in that PR and filed here.

## What

A mate side's frame is its base composed with its offset
(`crates/editor-core/src/mate/solve.rs`, `compose_offset`). For any
offset that is not a bit-exact identity, the frame's axis is the
composed map's third column re-minted through `derived_direction`
under the solve's `Band::linear(tol)`. That column is unit-length by
construction: rigid steps are rotations, literal steps are A6-admitted
rigid frames, and an authored side's literal is the `point_at` frame
of its vectors. So the decision weighs a length of one against
`(ε, Kε)`, whatever the parts' scale:

- it never refuses where `Kε < 1`;
- where `Kε ≥ 1` (`ε ≥ 0.1` at the default K), every authored side and
  every non-identity offset refuses `MateFault::Frame`.

Before the unit, an authored side's raw axis went through the witness
ladder at the doc's tolerance, so a long authored axis (`1e4`) cleared
a coarse band. `viewer/tests/tree_badges.rs`'s `snapshot_mate` relied on
that at ε = 16, and was rewritten to the part base with the empty
chain, which decides nothing.

Pinned by `mate::solve::tests::a_composed_axis_is_decided_against_the_length_band`:
a `1e4`-long authored axis composes under a band at `1e-3` and refuses
under one at `0.5`; the empty chain composes under both.

## What it wants

The composed frame is rigid by construction, so its axis needs no
length decision. The fix is a witness that survives composition: an
`OrthoFrame` transported by a rigid motion, or a `UnitVec3` mint that
carries a rotation's column. That is geom-core's ground (`linalg`), so
it is a design question. Alternatively the decision could be levered by
the mate's own scale, at the cost of needing the reach at replay.
Either way, the original `snapshot_mate` fixture (1e4 vectors at
ε = 16) should come back.
