---
id: quadrature-setup-is-re-derived-per-round-window
kind: issue
title: a certified face lane re-derives its round-independent setup on every round window
status: open
opened: 2026-09-12
---


## The finding

PERF-6 made the certified face lanes enterable over a `RoundWindow`
(`crates/geom-brep/src/props/quad.rs`), so tier 3's check 7 can stop at
the round its sign settles and a caller wanting the number can continue
from there. The ROUNDS compose exactly — a face run as `0..=k` then
`k+1..=...` evaluates the pieces an uninterrupted run evaluates, once
each, which `crates/sweep/tests/sign_certified_plus_v.rs` counts through
`k_stats` (`gate + refine == one`).

What does NOT compose is everything a lane does BEFORE its round loop.
A window enters at the lane's front door, so each entry re-derives that
face's round-independent setup:

- `nurbs_patch_face` (quad.rs, the block before its loop): the
  derivative grids `su`/`sv`/`suu`/`svv`, the global second-derivative
  hulls, the interior-knot lists, the position-magnitude bound, and
  `last_round_width_lo` over the schedule's LAST round's cut lists;
- `rational_patch_face`: the same, plus the homogeneous nets, the
  per-block hull grid and the `w`-uniform-in-v decision;
- `cylinder_cut_face`: the least of the three — its per-round work is
  the whole of it.

Measured (release, 4 vCPU, medians of 3): gate-then-continue is
1.3-1.8x one measurement in wall time (1766 vs 969 ms at `1e7*eps`;
3457 vs 2662 ms at `1e9*eps`) although the piece evaluations are
identical. That is the setup being paid twice, and it is paid once per
WINDOW, so a body whose sign settles late pays it once per round.

Against the two full quadratures a gate-then-measure caller used to
pay, 1.3-1.8x is still a saving. It is not the saving the composition
argument promises.

## What a fix is

A per-face prepared object the lane builds once and rounds read from —
so `RoundWindow` addresses rounds of a PREPARED face rather than
re-entering the lane. The natural shape is the one the round loops
already imply: everything above the loop becomes the constructor,
`round(r)` becomes the method, and `SignCertificate` holds the prepared
faces rather than re-flattening the body's loops.

Cost to weigh before it moves: the prepared object holds per-face
grids and hulls for every face of the body at once, where today one
face's setup is live at a time.

## Not in scope for a fix

The `cylinder_cut_face` lane, which has no meaningful setup, and the
exact per-span arm of `nurbs_patch_face`, which answers before any
round runs and is entered at most once per face by construction.
