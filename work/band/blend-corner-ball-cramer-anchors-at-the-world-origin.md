---
id: blend-corner-ball-cramer-anchors-at-the-world-origin
kind: issue
title: blend::arms::corner_ball solves its three-plane Cramer system from offsets to the world origin, so the corner ball's centre sits about u·|d|/det off its planes
status: open
opened: 2026-10-15
---


## The finding

Found in review of PR 5190 (`cleave/circle-pair-touch-on-both`), which
fixed the same class in `boolean::carrier_cross` (`common_line`).

`blend::arms::corner_ball` (`crates/sweep/src/blend/arms.rs:1039-1060`)
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
