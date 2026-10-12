---
id: a-cube-less-a-pinched-operand-piercing-its-face-refuses-the-split-cones-re-pair
kind: issue
title: A cube less a pinched operand whose cones pierce its face at one point refuses ZipCorrespondence in split_cones' re-pair (36 dbl lines)
status: open
opened: 2026-10-11
priority: P2
cost: M
---


## What

Found by `the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone`'s
lane (branch `flush/pinch-cluster`). The `sweep`
`join_pierce_runs_sweep::dbl_battery` (ignored, release) runs a pinched
operand against a cube: two corners touching only at `v`, the cube's
near face through `v`. Of its 2 160 lines, 36 refuse
`ZipCorrespondence { what: "a split section face does not pair one to
one" }`, raised by `crates/topo/src/boolean/zip.rs` `repair` after
`split_cones` has split the vertices at `v` per cone.

All 36 are `yx S`, the cube less the operand: 13 at `Ltop asym` seeds
2296 and 2959, and 23 at `asym asym` seeds 15, 225, 268 and 426
(directions 40 to 580). Replay one with
`DBL_POSE="asym 268 100 yx S" cargo test -p sweep --release --test all
dbl_one -- --ignored --nocapture`.

On the base (`weld_pinches` still welding the two pierces), the same 36
lines did not build right: the 23 `asym asym` lines built `OK BAD` with
three vertices at `v` for two cones (tier 3′ failing), and the 13
`Ltop asym` lines refused `ResultInvalid { RingMeetsOuter }`. So the typed refusal replaces a wrong body. It is
not a new failure of the split.

## Open

Not root-caused. The ring's root does not matter: the probe refused
the same way from every root `null::with_ring_root` sets (0 to 4) at
`asym 268 100`, `Ltop 2296 50` and `asym 15 40`. Read next: which of
the two sides' `split_side` merged section faces (`kef`) that the other
side's did not, at the cube's ring vertex or at the operand's vertices
at `v`.
