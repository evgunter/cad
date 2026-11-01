---
id: blend-corner-ball-cramer-anchors-at-the-world-origin
kind: issue
title: blend::arms::corner_ball solves its three-plane Cramer system from offsets to the world origin, so the corner ball's centre sits about u·|d|/det off its planes
status: closed
opened: 2026-10-19
priority: P2
cost: E
branch: band/arms-centre-placement
closed: 2026-10-19
---


## The finding

Found in review of PR 5190 (`cleave/circle-pair-touch-on-both`, closed
unmerged: FUSE's curve-pair reader replaced `carrier_cross::common_line`,
which had the same class).

`blend::arms::corner_ball` (`crates/sweep/src/blend/arms.rs:1038-1060`)
places the ball's centre by Cramer's rule on three planes. Each plane's
right-hand side is the vertex's offset from the world origin plus the
fold, `rhsᵢ = pᵢ·nᵢ + signed`, and the solve is
`c = (rhs₁·(n₂×n₃) + rhs₂·(n₃×n₁) + rhs₃·(n₁×n₂))/det`, with
`det = n₁·(n₂ × n₃)`.

- The numerator rounds by about `u·|rhs|`, so the centre sits about
  `u·|p|/det` off its three offset planes.
- That is a ball centre off its supports by more than ε wherever
  `|p|/det` passes `ε/u`: for a corner at `|p| ≈ 10`, a `det` under
  about `1e-7` at ε = 1e-9.
- **Measured (the reviewer's figures), `|d| ≈ 10`:** `1.1e-14` at
  `s = 0.1`, and `9.5e-9` at `s = 1e-7`.
- A corner of near-coplanar faces has a small `det`.

## What it needs

Solve from one of the corner's own vertices, say `p₁`:

- the right-hand sides become `(pᵢ − p₁)·nᵢ + signed`, small numbers;
- solve for `c − p₁`;
- add `p₁` back.

The error is then `u` of the corner's own size over `det`, not of its
distance from the origin. A ci row should pin a corner about 1 km from
the origin, at three ε.

## Closed

`corner_ball` solves for `c − p₁` with right-hand sides
`(pᵢ − p₁)·nᵢ + signed` and adds `p₁` back, so its rounding is of the
corner's own size. There is no shared three-plane solver to read
(topo's `offset_together::cramer` is private and has the same class:
filed `work/shell/topo-planar-offset-cramer-anchors-at-the-world-origin.md`).
`crates/sweep/tests/blend_arms_centre_placement.rs` pins a corner
about 1 km from the origin under a generic rotation, `det` from 0.5
to 1e-7, with the vertex thrice (as the doors hand it) and with three
distinct plane points: at `f64` the centre is within 1.5 ulps of
`|p| + r/det` of depth `r` in each plane (main: 229 at `det ≈ 1e-4`),
and the `Interval` enclosure is within 2.9 ulps of `|p| + r/det²`
(main: 126 at `det ≈ 1e-2`). The arm reads no ε, so the row is the
same at every ε row; it runs in the default suite and the eps rows.

Door reach: both doors (`open::planar`, `reach::corner_reach`) hand
the vertex thrice, so after the fix their right-hand sides are exactly
`signed`. Predicate 6 admits `det` down to `ε/r`, so on main a vertex
at `|p|` sat `u·|p|/det` off its planes — above ε at 1 km once `det`
fell under about `2e-4`. A door reaches that only with a corner whose
ball stands `r/det` (thousands of `r`) from the vertex inside the
body; no door row was built.
